FROM ubuntu:24.04@sha256:9cbed754112939e914291337b5e554b07ad7c392491dba6daf25eef1332a22e8
RUN apt-get update && apt install -y adb && rm -rf /var/lib/apt/lists/
COPY target/x86_64-unknown-linux-musl/release/adb-mqtt /usr/local/bin/adb-mqtt
CMD ["adb-mqtt"]
