FROM hub.lumigame.com/pentagon-base/upload_videos_s3:rust1.1 as builder

FROM hub.lumigame.com/pentagon-base/export_vars:1.0

COPY --from=builder /usr/local/bin/upload_videos /usr/local/bin/upload_videos

RUN apt-get update && apt-get install -y cifs-utils
RUN chmod +x /usr/local/bin/upload_videos