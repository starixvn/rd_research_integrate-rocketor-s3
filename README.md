# Video Upload and Checksum Services 🎥 ✨

A robust solution for managing video uploads and verification on AWS S3, featuring concurrent processing for optimal performance.

## 🌟 Features

- **Upload Videos Service**: Fast, concurrent video file uploads to S3
- **Checksum Service**: Reliable verification of uploaded video segments
- **Concurrent Processing**: Optimized performance with controlled parallel operations
- **Progress Tracking**: Real-time progress monitoring and detailed summaries

## 📋 Prerequisites

- Docker and Docker Compose
- AWS credentials
- Environment configuration

## 🚀 Quick Start

### 1. Environment Setup

Create a `.env` file in your project root:

```env
TRANSFORM_CONFIG=your_config_name
VARS_DATA={
  "services": [{
    "config_your_config_name": {
      "access_id": "your_aws_access_key",
      "secret_key": "your_aws_secret_key",
      "region": "your_aws_region",
      "bucket": "your_s3_bucket",
      "target_folder": "your_target_folder"
    }
  }]
}
```

### 2. Using in Docker

#### Upload Videos Service

```bash
# Build the container
docker-compose build upload_videos

# Run the service
docker-compose run upload_videos cargo run --bin upload_videos -- \
  --path /path/to/videos1 /path/to/videos2 \
  --exclude file1.mp4 file2.mp4 \
  --max-concurrent-uploads 10
```

#### Checksum Service

```bash
# Build the container
docker-compose build check_sum

# Run the service
docker-compose run check_sum cargo run --bin check_sum -- \
  --summary /path/to/summary.json \
  --max-concurrent-files 5 \
  --max-concurrent-segments 50
```

## 📝 Service Configuration

### Upload Videos Service

| Parameter                      | Description                | Default  |
| ------------------------------ | -------------------------- | -------- |
| `--path, -p`                   | Paths to video files       | Required |
| `--exclude, -e`                | Files to exclude           | Optional |
| `--max-concurrent-uploads, -c` | Maximum concurrent uploads | 10       |

### Checksum Service

| Parameter                       | Description                 | Default  |
| ------------------------------- | --------------------------- | -------- |
| `--summary, -s`                 | Path to summary JSON        | Required |
| `--max-concurrent-files, -f`    | Maximum concurrent files    | 5        |
| `--max-concurrent-segments, -m` | Maximum concurrent segments | 50       |

## 📊 File Formats

### Summary JSON Format

```json
{
  "folder1": 10,
  "folder2": 5
}
```

> Numbers represent the file count in each folder

## 📈 Output

- **Upload Service**: Generates `upload_summary.json` with detailed upload results
- **Both Services**: Provide real-time progress information in console

## 🔧 Docker Configuration

```yaml
services:
  upload_videos:
    image: hub.lumigame.com/pentagon-base/upload_videos_s3:rust1.0
    env_file:
      - .env

  check_sum:
    image: hub.lumigame.com/pentagon-base/check_sum_s3:latest
    env_file:
      - .env
```

## 🏗️ Project Structure

```
.
├── services/
│   ├── upload_videos/
│   └── check_sum/
├── libs/
│   └── s3-integration/
├── .env
├── docker-compose.yml
└── README.md
```

## 🤝 Contributing

Feel free to submit issues and enhancement requests!

## 📜 License

This project is proprietary and confidential.

## 🆘 Support

For support, please contact the development team.
