FROM hub.lumigame.com/docker-hub/library/rust:slim as builder
WORKDIR /app

COPY . .
RUN cargo build --release --bin upload_videos
RUN cargo build --release --bin export_vars

FROM hub.lumigame.com/docker-hub/library/debian:stable-slim
COPY --from=builder /app/target/release/upload_videos /usr/local/bin/upload_videos
COPY --from=builder /app/target/release/export_vars /usr/local/bin/export_vars
COPY --from=builder /app/export_env.sh /usr/local/bin/export_env.sh

RUN apt-get update && apt-get install -y cifs-utils
RUN chmod +x /usr/local/bin/upload_videos
RUN chmod +x /usr/local/bin/export_vars
RUN chmod +x /usr/local/bin/export_env.sh