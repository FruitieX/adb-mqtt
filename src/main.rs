use color_eyre::Result;

mod adb;
mod app_config;
mod mqtt;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let (mqtt_config, adb_config) = app_config::read_config()?;

    let mqtt_client = mqtt::init(&mqtt_config).await?;
    adb::init(mqtt_client, adb_config);

    tokio::signal::ctrl_c().await?;

    Ok(())
}
