use eyre::{eyre, Result};
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

    output.status.success().then_some(()).ok_or(eyre!(
        "Command '{} {}' failed, stderr: {}",
        cmd,
        args.into_iter()
            .map(|s| s.as_ref().to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(" "),
        String::from_utf8_lossy(&output.stderr)
    ))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}

pub async fn sleep_if_awake(addr: &str) -> Result<()> {
    run_cmd_timeout("adb", ["disconnect"], Duration::from_secs(5)).await?;
    run_cmd_timeout("adb", ["connect", addr], Duration::from_secs(5)).await?;
    run_cmd_timeout("adb", ["wait-for-device"], Duration::from_secs(5)).await?;
    let stdout = run_cmd_timeout(
        "adb",
        ["shell", "dumpsys activity | grep -c mWakefulness=Awake"],
        Duration::from_secs(1),
    )
    .await?;

    let is_on = stdout == "1\n";

    if is_on {
        run_cmd_timeout(
            "adb",
            ["shell", "input", "keyevent", "KEYCODE_POWER"],
            Duration::from_secs(5),
        )
        .await?;
    }

    Ok(())
}

pub fn init(mut mqtt_client: MqttClient, adb_config: AdbConfig) {
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
                Ok(())
            };

            if let Err(e) = result {
                eprintln!("Error: {:?}", e);
            }
        }
    });
}
