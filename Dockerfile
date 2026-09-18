FROM ubuntu:26.04@sha256:9559ceb7c21e528e233e8dff26a0fb2682f4094cce06176eeb075d87a22b31de
RUN apt-get update && apt install -y adb && rm -rf /var/lib/apt/lists/
COPY target/x86_64-unknown-linux-musl/release/adb-mqtt /usr/local/bin/adb-mqtt
CMD ["adb-mqtt"]
