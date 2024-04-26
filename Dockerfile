FROM ubuntu:24.04@sha256:562456a05a0dbd62a671c1854868862a4687bf979a96d48ae8e766642cd911e8
RUN apt-get update && apt install -y adb && rm -rf /var/lib/apt/lists/
COPY target/x86_64-unknown-linux-musl/release/adb-mqtt /usr/local/bin/adb-mqtt
CMD ["adb-mqtt"]
