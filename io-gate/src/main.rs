use clap::Parser;
use io_gate::comm;
use io_gate::config::Config;
use io_gate::homeassistant::{self, discovery, HomeAssistant};
use io_gate::message::{args::OutputState, Message};
use tokio::time::Duration;
use tracing::info;
use tracing_subscriber::fmt;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, default_value = "config.yaml")]
    config_path: String,

    // USB Connection
    #[arg(long, default_value = "/dev/ttyACM0")]
    port_name: String,
    #[arg(long, default_value_t = 115200)]
    baud_rate: u32,

    // MQTT connection
    #[arg(long)]
    mqtt_host: String,
    #[arg(long, default_value_t = 1883)]
    mqtt_port: u16,
    #[arg(long, default_value = "")]
    mqtt_username: String,
    #[arg(long, default_value = "")]
    mqtt_password: String,

    // Other
    #[arg(long, default_value = "io-gate")]
    device_name: String,
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

/// Perform initial configuration and device discovery.
async fn init_config(config: &Config, ha: &HomeAssistant) -> anyhow::Result<()> {
    for (device_name, cfg) in &config.devices {
        let message = discovery::new_device(&device_name, cfg.addr, cfg.outputs.count);
        ha.send(homeassistant::Outgoing::DiscoveryDevice(message))
            .await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_log();
    let args = Args::parse();

    let config = Config::from_file(&args.config_path)?;
    info!("Starting IO Gate. Args: {:?} Config: {:?}", args, config);

    let mut ha_init = homeassistant::Initiator::new(
        "io-gate-mqtt",
        &args.mqtt_host,
        args.mqtt_port,
        &args.mqtt_username,
        &args.mqtt_password,
    )
    .await;
    ha_init.set_topics("iogatetest", "iogatepub").await;
    let ha = ha_init.start().await;

    ha.send(homeassistant::Outgoing::Initial)
        .await
        .expect("Should send");

    init_config(&config, &ha).await?;

    let received = ha.recv().await;
    info!("Got {:?}", received);

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

            let msg = Message::SetOutput {
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
