use super::{Incoming, Outgoing};
use crate::consts;
use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
use rumqttc::{Event, Packet};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::{sync::Mutex, task};

use tracing::{info, warn, debug, error};

pub struct Initiator {
    client: AsyncClient,
    event_loop: EventLoop,
    base: String,
}

/// HA interfacing via MQTT
pub struct HomeAssistant {
    /// Outgoing event queue: things we sent to HA.
    outgoing: mpsc::Sender<Outgoing>,
    /// Incoming event queue: commands read from HA.
    incoming: Mutex<mpsc::Receiver<Incoming>>,
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
                    // This can be ON/OFF messaging
                    info!("RX message to {} with payload '{:?}'", msg.topic, msg.payload);
                    let topic = &msg.topic;
                    let parts: Vec<&str> = topic.split("/").collect();
                    // Maybe regexp instead?
                    if parts.len() == 5 && parts[0] == "smartenough" && parts[2] == "switch" && parts[4] == "set" {
                        // This is a command setting output to particular value.
                        let device = parts[1].parse::<u8>();
                        if device.is_err() {
                            warn!("Device address is not a 0-255 number: {}", parts[1]);
                            continue;
                        }
                        let device = device.unwrap();

                        let output= parts[3].parse::<u8>();
                        if output.is_err() {
                            warn!("Output index is not a 0-255 number: {}", parts[3]);
                            continue;
                        }
                        let output = output.unwrap();
                        let on = msg.payload == "ON";

                        let message = Incoming::SetOutput {
                            device,
                            output,
                            on,
                        };
                        queue.send(message).await
                    } else {
                        info!("Unknown topic - ignoring");
                        continue;
                    }
                }
                Ok(Event::Outgoing(_)) |
                Ok(Event::Incoming(Packet::PingResp)) |
                Ok(Event::Incoming(Packet::SubAck(_))) |
                Ok(Event::Incoming(Packet::PubAck(_))) => {
                    // Silence common messages
                    continue;
                },
                _ => {
                    info!("Received other message = {:?}", notification);
                    continue;
                }
            };
            if result.is_err() {
                error!(
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
                    Outgoing::Subscribe(topic) => {
                        let result = client
                            .subscribe(&topic, QoS::AtMostOnce)
                            .await;
                        if result.is_err() {
                            warn!("Unable to subscribe to a topic {}. Hard fail", topic);
                            panic!("Unable to continue");
                        }
                    }
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
                        client
                            .publish("hello/rumqtt", QoS::AtLeastOnce, false, raw)
                            .await
                            .expect("Test should work");
                    }
                    Outgoing::DiscoveryDevice(msg) => {
                        let topic = format!(
                            "{}/device/{}/config",
                            consts::HA_DISCOVERY_TOPIC,
                            msg.device.identifiers[0]
                        );
                        let payload = msg.serialize();
                        debug!("Sending discovery payload to {}: {}", topic, payload);
                        let result = client
                            .publish(topic, QoS::AtLeastOnce, false, payload)
                            .await;
                        if result.is_err() {
                            error!("Unable to publish discovery message {:?}", result);
                        }
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
    /// Receive incoming message (from MQTT). None means the HA reading loop
    /// finished.
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
