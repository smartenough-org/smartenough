use crate::message::MessageRaw;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time::Duration;
use tracing::info;

const PREAMBULE_LENGTH: usize = 2;
const CAN_MESSAGE_LENGTH: usize = 3 + 8;

/// Synchronization byte 1.
const SYNC_BYTE_1: u8 = 0x21; // !
/// Sync byte 2 for 3+8 byte content (CAN frame)
const SYNC_BYTE_2_CAN: u8 = 0x7C; // |

pub struct Comm {
    pub tx: mpsc::Sender<MessageRaw>,
    pub rx: mpsc::Receiver<MessageRaw>,
    pub reader: JoinHandle<anyhow::Result<()>>,
    pub writer: JoinHandle<anyhow::Result<()>>,
}

async fn reader(
    mut port: ReadHalf<tokio_serial::SerialStream>,
    channel: mpsc::Sender<MessageRaw>,
) -> anyhow::Result<()> {
    let mut buf = [0u8; 512];
    loop {
        let read_len: usize = match port.read(&mut buf).await {
            Ok(count) if count > 0 => count,
            Ok(_) => {
                println!("Probably disconnected? Length 0");
                anyhow::bail!("Disconnected");
            }
            Err(e) => {
                println!("Error {:?}", e);
                continue;
            }
        };

        // Simplified synchronization. Could be better.
        if buf[0] != SYNC_BYTE_1 {
            info!(
                "Synchronization 1 failed - preambule error. Skipping chunk: {:?}",
                &buf[0..read_len]
            );
            continue;
        }

        let body_len: usize = match buf[1] {
            SYNC_BYTE_2_CAN => {
                if read_len < 2 + CAN_MESSAGE_LENGTH {
                    println!(
                        "Invalid message length. Skipping chunk {:?}",
                        &buf[0..read_len]
                    );
                }
                CAN_MESSAGE_LENGTH
            }
            _ => {
                info!(
                    "Synchronization 2 failed - preambule error. Skipping chunk: {:?}",
                    &buf[0..read_len]
                );
                continue;
            }
        };

        if body_len + PREAMBULE_LENGTH > read_len {
            info!(
                "Synchronization failed - body {}, packet {} len. Skipping chunk: {:?}",
                body_len,
                read_len,
                &buf[0..read_len]
            );
            continue;
        }
        let packet = &buf[PREAMBULE_LENGTH..PREAMBULE_LENGTH + body_len];

        let addr = packet[0];
        let msg_type = packet[1];
        let length = packet[2] as usize;
        let data = &packet[3..3 + length];

        let raw = MessageRaw::from_bytes(addr, msg_type, data);
        //handle_incoming(raw).await?;
        channel.send(raw).await?;
    }
}

async fn writer(
    mut port: WriteHalf<tokio_serial::SerialStream>,
    mut channel: mpsc::Receiver<MessageRaw>,
) -> anyhow::Result<()> {
    loop {
        let mut buf = [0u8; 64];
        let msg = channel.recv().await;
        if let Some(msg) = msg {
            // TODO: Now it assumes CAN frames only.

            let (addr, msg_type) = msg.addr_type();
            let total_size = PREAMBULE_LENGTH + CAN_MESSAGE_LENGTH as usize;

            buf[0] = SYNC_BYTE_1;
            buf[1] = SYNC_BYTE_2_CAN;
            buf[2] = addr;
            buf[3] = msg_type;
            buf[4] = msg.length();
            buf[5..5 + msg.length() as usize].copy_from_slice(msg.data_as_array());

            let msg_buf = &buf[0..total_size];

            tokio::time::sleep(Duration::from_secs(1)).await;
            match port.write(msg_buf).await {
                Ok(size) => {
                    info!("USB TX: {} bytes: {:02x?}", size, msg_buf);
                }
                Err(err) => {
                    anyhow::bail!("Error while sending to port {:?}", err);
                }
            }
        } else {
            return Ok(());
        }
    }
}

#[tracing::instrument]
pub async fn run(port_name: String, baud_rate: u32) -> anyhow::Result<Comm> {
    // -> anyhow::Result<(mpsc::Sender<MessageRaw>, mpsc::Receiver<MessageRaw>)> {
    let builder = tokio_serial::new(port_name, baud_rate);
    let stream = tokio_serial::SerialStream::open(&builder)?;
    let (port_read, port_write) = tokio::io::split(stream);

    let (out_tx, out_rx) = mpsc::channel(15);
    let (in_tx, in_rx) = mpsc::channel(15);
    let reader = reader(port_read, in_tx);
    let writer = writer(port_write, out_rx);

    let reader_handle = tokio::spawn(reader);
    let writer_handle = tokio::spawn(writer);

    Ok(Comm {
        tx: out_tx,
        rx: in_rx,
        writer: writer_handle,
        reader: reader_handle,
    })
    // tokio::try_join!(reader, writer)?;
    //Ok((out_tx, in_rx))
}
