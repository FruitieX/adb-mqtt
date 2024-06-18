FROM ubuntu:24.04@sha256:2e863c44b718727c860746568e1d54afd13b2fa71b160f5cd9058fc436217b30
RUN apt-get update && apt install -y adb && rm -rf /var/lib/apt/lists/
COPY target/x86_64-unknown-linux-musl/release/adb-mqtt /usr/local/bin/adb-mqtt
CMD ["adb-mqtt"]
