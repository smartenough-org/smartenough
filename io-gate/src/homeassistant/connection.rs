use super::{Incoming, Outgoing};
use crate::consts;
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use rumqttc::{Event, Packet};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::{sync::Mutex, task};

use tracing::{info, warn};

/// HA interfacing via MQTT
pub struct HomeAssistant {
    /// Outgoing event queue: things we sent to HA.
    outgoing: mpsc::Sender<Outgoing>,
    /// Incoming event queue: commands read from HA.
    incoming: Mutex<mpsc::Receiver<Incoming>>,
}

pub struct Initiator {
    client: AsyncClient,
    event_loop: EventLoop,
    base: String,
}

impl Initiator {
    pub async fn new(id: &str, host: &str, port: u16, username: &str, password: &str) -> Self {
        let mut mqttoptions = MqttOptions::new(id, host, port);
        mqttoptions.set_keep_alive(Duration::from_secs(5));
        mqttoptions.set_credentials(username, password);

        let (client, mut event_loop) = AsyncClient::new(mqttoptions, 10);
        client
            .subscribe("hello/rumqtt", QoS::AtMostOnce)
            .await
            .unwrap();

        // Fail early if parameters are invalid.
        let initial_result = event_loop.poll().await;
        if initial_result.is_err() {
            warn!("Initial connection to MQTT failed. Check connection parameters");
            panic!("Unable to contact MQTT");
        }

        Initiator {
            client,
            event_loop,
            base: "io-gate/".into(),
        }
    }

    /// Configure topics prefixes.
    pub async fn set_topics(&mut self, base: &str, discover_prefix: &str) {
        // TODO: Handle when reading state changes from HA
        self.client
            .subscribe(discover_prefix, QoS::AtMostOnce)
            .await
            .unwrap();
        self.base = base.to_string();
    }

    async fn receiver(mut event_loop: EventLoop, queue: mpsc::Sender<Incoming>) {
        loop {
            let notification = event_loop.poll().await;
            let result = match notification {
                Ok(Event::Incoming(Packet::Publish(msg))) => {
                    info!("Payload: {:?}", msg.payload);
                    info!("Received incoming-publish = {:?}", msg);
                    queue.send(Incoming::RawTest(msg.payload.to_vec())).await
                }
                _ => {
                    info!("Received other = {:?}", notification);
                    continue;
                }
            };
            if result.is_err() {
                warn!(
                    "Error while sending received message to queue: {:?}. Quitting loop",
                    result
                );
                return;
            }
        }
    }

    async fn sender(client: AsyncClient, mut queue: mpsc::Receiver<Outgoing>) {
        loop {
            if let Some(command) = queue.recv().await {
                match command {
                    Outgoing::Initial => {
                        client
                            .publish(
                                "smartenough/status",
                                QoS::AtLeastOnce,
                                false,
                                "daemon started",
                            )
                            .await
                            .expect("Initial message should publish fine");
                    }
                    Outgoing::RawTest(raw) => {
                        let result = client
                            .publish("hello/rumqtt", QoS::AtLeastOnce, false, raw)
                            .await;
                        info!("Published and returned {:?}", result);
                    }
                    Outgoing::DiscoveryDevice(msg) => {
                        let topic = format!(
                            "{}/device/io-gate-{}/config",
                            consts::HA_DISCOVERY_TOPIC,
                            msg.device.identifiers[0]
                        );
                        let payload = msg.to_string();
                        info!("Sending discovery payload to {}: {}", topic, payload);
                        let result = client
                            .publish(topic, QoS::AtLeastOnce, false, payload)
                            .await;
                        info!("Published and returned {:?}", result);
                    }
                }
            } else {
                // Channel end closed - quit.
                return;
            }
        }
    }

    pub async fn start(self) -> HomeAssistant {
        let (out_sender, out_receiver) = mpsc::channel::<Outgoing>(10);
        let (in_sender, in_receiver) = mpsc::channel::<Incoming>(10);
        task::spawn(Self::receiver(self.event_loop, in_sender));
        task::spawn(Self::sender(self.client, out_receiver));

        HomeAssistant {
            outgoing: out_sender,
            incoming: Mutex::new(in_receiver),
        }
    }
}

impl HomeAssistant {
    pub async fn recv(&self) -> Option<Incoming> {
        // Receive incoming messages.
        let mut incoming = self.incoming.lock().await;
        incoming.recv().await
    }

    pub async fn send(&self, msg: Outgoing) -> anyhow::Result<()> {
        self.outgoing.send(msg).await?;
        Ok(())
    }
}
