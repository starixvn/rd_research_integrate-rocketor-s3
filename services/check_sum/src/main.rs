use clap::{command, Parser};
use s3_integration::{config_client, get_object_as_string, Config};
mod generate_sha256;
mod parse_m3u8_segments;
mod read_summary;
use futures::stream::{self, StreamExt};

use generate_sha256::generate_sha256;
use parse_m3u8_segments::parse_m3u8_segments;
use read_summary::read_summary;

#[derive(Parser)]
#[command(name = "read_objects")]
#[command(about = "A simple program to read objects from s3", long_about = None)]
struct Args {
    #[arg(short, long)]
    summary: String,

    #[arg(short = 'f', long, default_value = "5")]
    max_concurrent_files: usize,

    #[arg(short = 'm', long, default_value = "50")]
    max_concurrent_segments: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    dotenv::from_filename(".env").ok();
    let env_name = std::env::var("TRANSFORM_CONFIG").expect("TRANSFORM_CONFIG is not set");
    let vars_data = std::env::var("VARS_DATA").expect("VARS_DATA is not set");
    let vars_data: serde_json::Value = serde_json::from_str(&vars_data).unwrap();
    let config = vars_data["services"][0][format!("config_{}", env_name)].clone();
    let config: Config = serde_json::from_value(config).expect("Failed to deserialize config");
    std::env::set_var("AWS_ACCESS_KEY_ID", &config.access_id);
    std::env::set_var("AWS_SECRET_ACCESS_KEY", &config.secret_key);
    std::env::set_var("AWS_REGION", &config.region);
    let client = config_client().await;
    let summary = read_summary(&args.summary)
        .await
        .expect("Failed to read summary");
    let sum_values = summary.values().sum::<i32>();
    println!("Total files to check: {}", sum_values);
    // Process files concurrently with a maximum of 5 simultaneous files
    for (key, value) in summary {
        let files_futures = (0..value).map(|i| {
            let client = &client;
            let config = &config;
            let key = key.clone();
            async move {
                let object_key = generate_sha256(format!("{}-{}", key, i).as_str());

                // Process both resolutions concurrently
                let resolution_futures = ["240p", "720p"].into_iter().map(|res| {
                    let client = client.clone();
                    let config = config.clone();
                    let object_key = object_key.clone();
                    async move {
                        let object = get_object_as_string(
                            &client,
                            format!("{}/{}_{}.m3u8", &config.target_folder, &object_key, res)
                                .as_str(),
                            &config.bucket,
                        )
                        .await
                        .expect(format!("Failed to get object: {}", object_key).as_str());

                        let segments = parse_m3u8_segments(&object);

                        // Process segments concurrently
                        stream::iter(segments)
                            .map(|segment| {
                                let client = client.clone();
                                let config = config.clone();
                                async move {
                                    client
                                        .head_object()
                                        .bucket(&config.bucket)
                                        .key(
                                            format!("{}/{}", &config.target_folder, segment)
                                                .as_str(),
                                        )
                                        .send()
                                        .await
                                        .expect(
                                            format!("Failed to get object: {}", segment).as_str(),
                                        );
                                }
                            })
                            .buffer_unordered(args.max_concurrent_segments)
                            .collect::<Vec<_>>()
                            .await;
                    }
                });

                futures::future::join_all(resolution_futures).await;
                println!("Checked file {} of {} in {}", i + 1, value, key);
            }
        });

        // Process files with controlled concurrency
        stream::iter(files_futures)
            .buffer_unordered(args.max_concurrent_files)
            .collect::<Vec<_>>()
            .await;
    }
}
