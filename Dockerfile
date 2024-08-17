FROM ubuntu:24.04@sha256:a68b7d8b873b955e10337a18c79334a9d95dced834f85714890aa5ed7f6b2692
RUN apt-get update && apt install -y adb && rm -rf /var/lib/apt/lists/
COPY target/x86_64-unknown-linux-musl/release/adb-mqtt /usr/local/bin/adb-mqtt
CMD ["adb-mqtt"]
