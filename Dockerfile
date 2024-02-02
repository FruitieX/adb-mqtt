FROM ubuntu:22.04@sha256:e9569c25505f33ff72e88b2990887c9dcf230f23259da296eb814fc2b41af999
RUN apt-get update && apt install -y adb && rm -rf /var/lib/apt/lists/
COPY target/x86_64-unknown-linux-musl/release/adb-mqtt /usr/local/bin/adb-mqtt
CMD ["adb-mqtt"]
