use anyhow::Result;
use clap::Parser;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::time::Duration;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "/dev/ttyACM0")]
    port_name: String,
    #[arg(long, default_value_t = 115200)]
    baud_rate: u32,
}

const SYNC_BYTE_1: u8 = 0x21;
const SYNC_BYTE_2: u8 = 0x7C;

async fn handle_incoming(packet: &[u8]) -> Result<()> {
    println!("Got synchronized. Buf: {:?}", &packet);
    Ok(())
}

async fn reader(mut port: ReadHalf<tokio_serial::SerialStream>) -> Result<()> {
    let mut buf = [0u8; 512];
    loop {
        match port.read(&mut buf).await {
            Ok(count) if count > 0 => {
                // Simplified synchronization. Could be better.
                if buf[0] != SYNC_BYTE_1 || buf[1] != SYNC_BYTE_2 {
                    println!(
                        "Synchronization failed - preambule error. Skipping chunk: {:?}",
                        &buf[0..count]
                    );
                    continue;
                }
                let length = buf[2];
                if (2 + 1 + length) as usize != count {
                    println!(
                        "Synchronization failed - length {} != {} error. Skipping chunk: {:?}",
                        length,
                        count,
                        &buf[0..count]
                    );
                    continue;
                }
                let packet = &buf[3..count];
                handle_incoming(packet).await?;
            }
            Ok(_) => {
                println!("Probably disconnected? Length 0");
                anyhow::bail!("Disconnected");
            }
            Err(e) => {
                println!("Error {:?}", e);
            }
        }
    }
}

async fn writer(mut port: WriteHalf<tokio_serial::SerialStream>) -> Result<()> {
    let mut message = [0u8; 64];
    loop {
        message[0] = 0x21; // !
        message[1] = 0x7C; // |
        message[2] = 4; // Packet size
                        // Addr
        message[3] = 0x01;
        // Type
        message[4] = 0x01;
        // internal size
        message[5] = 0x01;
        // First byte of contents.
        message[6] = 0xAA;

        tokio::time::sleep(Duration::from_secs(1)).await;
        match port.write(&message[0..7]).await {
            Ok(size) => {
                println!("Sent {} bytes: {:?}", size, &message[0..7]);
            }
            Err(err) => {
                println!("Error while sending to port {:?}", err);
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let builder = tokio_serial::new(&args.port_name, args.baud_rate);
    let stream = tokio_serial::SerialStream::open(&builder)?;
    let (port_read, port_write) = tokio::io::split(stream);

    let reader = reader(port_read);
    let writer = writer(port_write);

    println!("Hello, world!");
    tokio::try_join!(reader, writer)?;
    Ok(())
}
