FROM ubuntu:26.04@sha256:065aa8626841f16057b78b027b464cd59c4fac36c204c72dcbf48d66f5fa8a96
RUN apt-get update && apt install -y adb && rm -rf /var/lib/apt/lists/
COPY target/x86_64-unknown-linux-musl/release/adb-mqtt /usr/local/bin/adb-mqtt
CMD ["adb-mqtt"]
