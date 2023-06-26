use eyre::Result;
use rand::{distributions::Alphanumeric, Rng};
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

pub async fn init(mqtt_config: &MqttConfig) -> Result<MqttClient> {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let mut options = MqttOptions::new(
        format!("{}-{}", mqtt_config.id.clone(), random_string),
        mqtt_config.host.clone(),
        mqtt_config.port,
    );
    options.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(options, 10);
    client
        .subscribe(format!("{}/set", mqtt_config.topic), QoS::AtMostOnce)
        .await?;

    let (tx, rx) = tokio::sync::watch::channel(None);

    task::spawn(async move {
        loop {
            let notification = eventloop.poll().await;

            let res = (|| async {
                if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(msg)) = notification? {
                    let device: MqttDevice = serde_json::from_slice(&msg.payload)?;
                    tx.send(Some(device))?;
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

    Ok(MqttClient {
        client,
        rx,
        topic: mqtt_config.topic.clone(),
    })
}
