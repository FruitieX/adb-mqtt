FROM ubuntu:26.04@sha256:e153663f92c94118ff22a5dc397b59b351ffd695480566debb5850e017e5937a
RUN apt-get update && apt install -y adb && rm -rf /var/lib/apt/lists/
COPY target/x86_64-unknown-linux-musl/release/adb-mqtt /usr/local/bin/adb-mqtt
CMD ["adb-mqtt"]
