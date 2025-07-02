FROM ubuntu:24.04@sha256:440dcf6a5640b2ae5c77724e68787a906afb8ddee98bf86db94eea8528c2c076
RUN apt-get update && apt install -y adb && rm -rf /var/lib/apt/lists/
COPY target/x86_64-unknown-linux-musl/release/adb-mqtt /usr/local/bin/adb-mqtt
CMD ["adb-mqtt"]
