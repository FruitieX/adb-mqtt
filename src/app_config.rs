use color_eyre::Result;
use eyre::{eyre, WrapErr};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MqttConfig {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub topic: String,
}

#[derive(Deserialize, Debug)]
pub struct AdbConfig {
    pub ip: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub mqtt: MqttConfig,
    pub device: AdbConfig,
}

pub fn read_config() -> Result<(MqttConfig, AdbConfig)> {
    let builder = config::Config::builder();

    let root = std::env::current_dir().unwrap();
    let sample_path = root.join("Settings.toml.example");

    let path = root.join("Settings.toml");

    if !path.exists() && std::env::var("SKIP_SAMPLE_CONFIG").is_err() {
        println!("Settings.toml not found, generating sample configuration.");
        println!("Set SKIP_SAMPLE_CONFIG environment variable to opt out of this behavior.");
        std::fs::copy(sample_path, path).unwrap();
    }

    let builder = builder.add_source(config::File::with_name("Settings"));
    let settings = builder.build()?;

    let config: Config = settings.try_deserialize().wrap_err_with(|| {
        eyre!("Failed to deserialize config, compare your config file to Settings.toml.example!")
    })?;

    let mqtt_config = config.mqtt;
    let adb_config = config.device;

    Ok((mqtt_config, adb_config))
}
