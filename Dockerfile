FROM ubuntu:24.04@sha256:35b7fc72eb7c652dc1f4e5bfbdb9cdb308c3a6b1b96abc61317b931007b9aac8
RUN apt-get update && apt install -y adb && rm -rf /var/lib/apt/lists/
COPY target/x86_64-unknown-linux-musl/release/adb-mqtt /usr/local/bin/adb-mqtt
CMD ["adb-mqtt"]
