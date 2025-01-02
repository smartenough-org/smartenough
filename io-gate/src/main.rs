use anyhow::Result;
use clap::Parser;
use io_gate::comm;
use io_gate::message::{Message, MessageRaw, args::OutputState};
use tokio::time::Duration;
use tracing_subscriber::fmt;
use tracing::{debug, warn, info};


#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "/dev/ttyACM0")]
    port_name: String,
    #[arg(long, default_value_t = 115200)]
    baud_rate: u32,
}

fn init_log() {
    let timer = fmt::time::ChronoLocal::new("%H:%M:%S%.3f".to_string());

    // Configure a custom event formatter
    let format = fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_source_location(true)
        .with_timer(timer)
        .compact();

    fmt()
        .event_format(format)
        .with_max_level(tracing::Level::TRACE)
        .init();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_log();
    let args = Args::parse();

    info!("Starting IO Gate {:?}", args);

    let mut comm = comm::run(args.port_name, args.baud_rate).await?;

    println!("Hello, world!");
    tokio::spawn(async move {
        // TEMP: Receiver
        loop {
            let msg = comm.rx.recv().await;
            info!("USB RX. Buf: {:?}", &msg);
            if let Some(msg) = msg {
                let msg = Message::from_raw(msg);
                info!("Parsed to {:?}", msg);
            } else {
                break;
            }
        }
    });

    tokio::spawn(async move {
        // TEMP: Transmitter
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;

            let msg = Message::SetOutput{
                output: 5,
                state: OutputState::Toggle,
            };
            let addr = 2;
            let raw = msg.to_raw(addr);
            info!("Sending RAW message over USB {:?}", raw);
            if comm.tx.send(raw).await.is_err() {
                break;
            }
        }
    });

    // Wait for tasks.
    let (reader_ret, writer_ret) = tokio::try_join!(comm.reader, comm.writer)?;
    reader_ret?;
    writer_ret?;
    Ok(())
}
