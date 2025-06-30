FROM hub.lumigame.com/docker-hub/library/rust:slim as builder
WORKDIR /app

COPY . .
RUN cargo build --release --bin upload_videos

FROM hub.lumigame.com/docker-hub/library/debian:stable-slim
COPY --from=builder /app/target/release/upload_videos /usr/local/bin/upload_videos

RUN apt-get update && apt-get install -y cifs-utils
RUN mkdir -p /mnt/share_folder
RUN chmod +x /usr/local/bin/upload_videos