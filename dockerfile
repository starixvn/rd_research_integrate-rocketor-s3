FROM hub.lumigame.com/pentagon-base/upload_videos_s3:rust1.1 as builder

COPY /usr/local/bin/upload_videos /upload_videos

FROM hub.lumigame.com/pentagon-base/export_vars:1.0

COPY --from=builder /upload_videos /usr/local/bin/upload_videos

RUN apt-get update && apt-get install -y cifs-utils
RUN chmod +x /usr/local/bin/upload_videos
RUN chmod +x /usr/local/bin/export_env.sh