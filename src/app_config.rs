use color_eyre::Result;
use eyre::{eyre, WrapErr};
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct MqttConfig {
    pub id: String,
    pub host: String,
    pub port: u16,
    pub topic: String,
    /// Broker credentials. The client connects anonymously when `username` is unset.
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct AdbConfig {
    pub ip: String,
    pub name: String,
    pub poll_rate: Option<u64>,
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

    let config = parse_config(builder.add_source(config::File::with_name("Settings")))?;

    let mqtt_config = config.mqtt;
    let adb_config = config.device;

    Ok((mqtt_config, adb_config))
}

fn parse_config(builder: config::ConfigBuilder<config::builder::DefaultState>) -> Result<Config> {
    let settings = builder.build()?;

    settings.try_deserialize().wrap_err_with(|| {
        eyre!("Failed to deserialize config, compare your config file to Settings.toml.example!")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_example_config() {
        let builder = config::Config::builder().add_source(config::File::from_str(
            include_str!("../Settings.toml.example"),
            config::FileFormat::Toml,
        ));

        let config = parse_config(builder).unwrap();

        assert_eq!(config.mqtt.id, "adb-mqtt");
        assert_eq!(config.mqtt.port, 1883);
        assert_eq!(config.mqtt.topic, "home/devices/adb/+");
        assert_eq!(config.device.ip, "1.2.3.4");
        assert_eq!(config.device.poll_rate, None);
    }
}
