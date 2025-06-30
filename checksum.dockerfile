FROM hub.lumigame.com/docker-hub/library/rust:slim as builder
WORKDIR /app

COPY . .
RUN cargo build --release --bin check_sum

FROM hub.lumigame.com/docker-hub/library/debian:stable-slim
COPY --from=builder /app/target/release/check_sum /usr/local/bin/check_sum

RUN chmod +x /usr/local/bin/check_sum
