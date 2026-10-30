#![allow(clippy::redundant_closure_call)]

use eyre::Result;
use rand::{distr::Alphanumeric, RngExt};
use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::{sync::watch::Receiver, task};

use crate::app_config::MqttConfig;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct MqttDevice {
    pub id: String,
    pub name: Option<String>,
    pub power: Option<bool>,
}

#[derive(Clone)]
pub struct MqttClient {
    pub client: AsyncClient,
    pub rx: Receiver<Option<MqttDevice>>,
    pub topic: String,
}

/// Builds a unique MQTT client id by appending a random suffix to `id`.
fn client_id(id: &str) -> String {
    let random_string: String = rand::rng()
        .sample_iter(Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    format!("{id}-{random_string}")
}

pub async fn init(mqtt_config: &MqttConfig) -> Result<MqttClient> {
    let mut options = MqttOptions::new(
        client_id(&mqtt_config.id),
        mqtt_config.host.clone(),
        mqtt_config.port,
    );
    options.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(options, 10);

    let (tx, rx) = tokio::sync::watch::channel(None);

    {
        let client = client.clone();
        let mqtt_config = mqtt_config.clone();

        task::spawn(async move {
            loop {
                let notification = eventloop.poll().await;

                let res = (|| async {
                    match notification? {
                        rumqttc::Event::Incoming(rumqttc::Packet::ConnAck(_)) => {
                            client
                                .subscribe(format!("{}/set", mqtt_config.topic), QoS::AtMostOnce)
                                .await?;
                        }
                        rumqttc::Event::Incoming(rumqttc::Packet::Publish(msg)) => {
                            let device: MqttDevice = serde_json::from_slice(&msg.payload)?;
                            tx.send(Some(device))?;
                        }
                        _ => {}
                    }

                    Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
                })()
                .await;

                if let Err(e) = res {
                    eprintln!("MQTT error: {:?}", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        });
    }

    Ok(MqttClient {
        client,
        rx,
        topic: mqtt_config.topic.clone(),
    })
}

