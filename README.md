# adb-mqtt

This program subscribes to an MQTT topic e.g. `home/devices/adb/<device_id>/set` and allows turning off an Android device
that is configured to use adb over network.

## Running

Make sure you have a recent version of Rust installed.
Android's platform-tools are required as well.
There's a docker image provided with all necessary dependencies installed.

1. Clone this repo
2. Copy Settings.toml.example -> Settings.toml
3. Configure Settings.toml to match your setup (see below)
4. `cargo run`

## MQTT protocol

MQTT messages use the following JSON format:

```
{
  "power": false,
}
```