use clap::{command, Parser};
use futures::stream::{self, StreamExt};
use s3_integration::{config_client, get_all_files, get_all_objects, upload_file, Config};
use serde_json::json;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Parser)]
#[command(name = "read_objects")]
#[command(about = "A simple program to read objects from s3", long_about = None)]
struct Args {
    #[arg(short, long)]
    exclude: Vec<String>,

    #[arg(short, long)]
    path: Vec<String>,

    #[arg(short, long, default_value = "10")]
    max_concurrent_uploads: usize,

    #[arg(short, long, default_value = "upload_summary.json")]
    report_file: String,
}

#[tokio::main]
async fn main() {
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
    let (all_objects_uploaded, mut all_files_uploaded) =
        get_all_objects(&client, &config).await.unwrap();
    let args = Args::parse();
    all_files_uploaded.extend(args.exclude);
    let mut upload_files: HashMap<String, String> = HashMap::new();
    for path in args.path {
        let files = get_all_files(&path, &all_files_uploaded).await.unwrap();
        upload_files.extend(files);
    }

    let upload_filenames = upload_files.keys().cloned().collect::<Vec<String>>();

    let data = json!({
        "upload_files": upload_filenames,
        "existing_objects": all_objects_uploaded,
        "existing_files": all_files_uploaded
    });
    let mut file = File::create(args.report_file)
        .await
        .expect("Failed to create file");
    file.write_all(serde_json::to_string(&data).unwrap().as_bytes())
        .await
        .expect("Failed to write to file");

    println!("Total files to upload: {}", upload_files.len());
    let total_files = upload_files.len();
    let uploaded_count = Arc::new(AtomicUsize::new(0));

    // Process uploads concurrently with a maximum of 10 simultaneous uploads
    stream::iter(upload_files)
        .map(|(file, filepath)| {
            let client = &client;
            let config = &config;
            let uploaded_count = Arc::clone(&uploaded_count);
            async move {
                upload_file(
                    client,
                    &config.bucket,
                    &config.target_folder,
                    &file,
                    &filepath,
                )
                .await
                .expect(format!("Failed to upload file: {}", file).as_str());

                let current = uploaded_count.fetch_add(1, Ordering::SeqCst) + 1;
                println!("Uploaded {} of {} files", current, total_files);
            }
        })
        .buffer_unordered(args.max_concurrent_uploads)
        .collect::<Vec<_>>()
        .await;

    println!("All uploads completed!");
}
