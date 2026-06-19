FROM ubuntu:26.04@sha256:53958ec7b67c2c9355df922dd08dbf0360611f8c3cdb656875e81873db9ffdba
RUN apt-get update && apt install -y adb && rm -rf /var/lib/apt/lists/
COPY target/x86_64-unknown-linux-musl/release/adb-mqtt /usr/local/bin/adb-mqtt
CMD ["adb-mqtt"]
