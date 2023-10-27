use eyre::{eyre, Result};
use rumqttc::QoS;
use std::time::Duration;
use tokio::process::Command;

use crate::{
    app_config::AdbConfig,
    mqtt::{MqttClient, MqttDevice},
};

async fn run_cmd_timeout<I, S>(cmd: &str, args: I, timeout: Duration) -> Result<String>
where
    I: IntoIterator<Item = S> + Clone,
    S: AsRef<std::ffi::OsStr>,
{
    let output =
        tokio::time::timeout(timeout, Command::new(cmd).args(args.clone()).output()).await??;

    let _result = output
        .status
        .success()
        .then_some(())
        .ok_or(eyre!(
            "Command '{} {}' failed, stdout: {} stderr: {}",
            cmd,
            args.into_iter()
                .map(|s| s.as_ref().to_string_lossy().to_string())
                .collect::<Vec<_>>()
                .join(" "),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ))
        .map_err(|e| e.to_string());

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}

pub async fn connect(addr: &str) -> Result<()> {
    // Defaults to port 5555 if not specified
    let (ip, port) = match addr.split_once(':') {
        Some((ip, port)) => (ip, port.parse::<u16>().unwrap_or(5555)),
        None => (addr, 5555),
    };

    let address = format!("{}:{}", ip, port);

    run_cmd_timeout("adb", ["disconnect"], Duration::from_secs(5)).await?;
    run_cmd_timeout("adb", ["connect", address.as_str()], Duration::from_secs(5)).await?;
    run_cmd_timeout("adb", ["wait-for-device"], Duration::from_secs(5)).await?;

    Ok(())
}

pub async fn is_awake(addr: &str) -> Result<bool> {
    connect(addr).await?;
    let stdout = run_cmd_timeout(
        "adb",
        ["shell", "dumpsys activity | grep -c mWakefulness=Awake"],
        Duration::from_secs(1),
    )
    .await?;

    let is_on = stdout == "1\n";

    Ok(is_on)
}

pub async fn sleep_if_awake(addr: &str) -> Result<()> {
    let is_on = is_awake(addr).await?;

    if is_on {
        toggle_sleep().await?;
    }

    Ok(())
}

pub async fn wake_if_asleep(addr: &str) -> Result<()> {
    let is_on = is_awake(addr).await?;

    if !is_on {
        toggle_sleep().await?;
    }

    Ok(())
}

pub async fn toggle_sleep() -> Result<()> {
    // Toggles the power state of the device between sleep and awake
    // Note that you must have the device connected via adb for this to work
    run_cmd_timeout(
        "adb",
        ["shell", "input", "keyevent", "KEYCODE_POWER"],
        Duration::from_secs(5),
    )
    .await?;

    Ok(())
}

pub fn init(mut mqtt_client: MqttClient, adb_config: AdbConfig) {
    // create tokio task that will update the device state to mqtt when it changes
    let cfg = adb_config.clone();
    tokio::spawn(async move {
        loop {
            connect(&cfg.ip).await.unwrap();
            let is_on = is_awake(&cfg.ip).await.unwrap_or(false);

            let device = MqttDevice {
                id: cfg.ip.clone(),
                name: Some(cfg.name.clone()),
                power: Some(is_on),
            };

            let topic = mqtt_client
                .topic
                .clone()
                .replace('+', device.clone().id.as_str());

            mqtt_client
                .client
                .publish(
                    topic,
                    QoS::AtMostOnce,
                    false,
                    serde_json::to_vec(&device).unwrap(),
                )
                .await
                .unwrap();

            tokio::time::sleep(Duration::from_secs(cfg.poll_rate.unwrap_or(5))).await;
        }
    });

    tokio::spawn(async move {
        loop {
            mqtt_client
                .rx
                .changed()
                .await
                .expect("Expected rx channel never to close");
            let device = mqtt_client.rx.borrow().clone();

            let result = if let Some(MqttDevice {
                power: Some(false), ..
            }) = device
            {
                sleep_if_awake(&adb_config.ip).await
            } else {
                wake_if_asleep(&adb_config.ip).await
            };

            if let Err(e) = result {
                eprintln!("Error: {:?}", e);
            }
        }
    });
}
