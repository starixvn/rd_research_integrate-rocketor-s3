use aws_sdk_s3::Client;
use std::{collections::HashSet, error::Error};

use crate::Config;

pub async fn get_all_objects(
    client: &Client,
    config: &Config,
) -> Result<(HashSet<String>, HashSet<String>), Box<dyn Error>> {
    let mut set_of_keys: HashSet<String> = HashSet::new();
    let mut video_keys: HashSet<String> = HashSet::new();
    let mut continuation_token = None;

    loop {
        let mut request = client
            .list_objects_v2()
            .bucket(&config.bucket)
            .prefix(&config.target_folder)
            .max_keys(1000); // Maximum allowed per request

        if let Some(token) = continuation_token {
            request = request.continuation_token(token);
        }

        let response = request.send().await.expect("Failed to get objects");

        for object in response.contents() {
            let key = object.key().expect("Failed to get key");
            if !key.is_empty() {
                let replaced = key.replace(&format!("{}/", config.target_folder), "");
                let parts: Vec<&str> = replaced.split(".").collect();
                let parts: Vec<&str> = parts[0].split("_").collect();
                let filename = parts[0];
                if filename.len() > 0 {
                    set_of_keys.insert(filename.to_string());
                    video_keys.insert(replaced.clone());
                }
            }
        }

        match response.next_continuation_token() {
            Some(token) => continuation_token = Some(token.to_string()),
            None => break,
        }
    }

    Ok((set_of_keys, video_keys))
}
