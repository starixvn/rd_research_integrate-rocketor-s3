FROM hub.lumigame.com/docker-hub/library/rust:slim as builder
WORKDIR /app

COPY . .
RUN cargo build --release --bin export_vars

FROM hub.lumigame.com/docker-hub/library/debian:stable-slim
COPY --from=builder /app/target/release/export_vars /usr/local/bin/export_vars
COPY --from=builder /app/services/export_vars/export_env.sh /usr/local/bin/export_env.sh

RUN chmod +x /usr/local/bin/export_vars
RUN chmod +x /usr/local/bin/export_env.sh