FROM ubuntu:24.04@sha256:ee6860ab126bb8291052401af00acad20d69c16e46579a47dac1c57cd4688446
RUN apt-get update && apt install -y adb && rm -rf /var/lib/apt/lists/
COPY target/x86_64-unknown-linux-musl/release/adb-mqtt /usr/local/bin/adb-mqtt
CMD ["adb-mqtt"]
