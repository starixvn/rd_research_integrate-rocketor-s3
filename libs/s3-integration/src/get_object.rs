use aws_sdk_s3::Client;

pub async fn get_object(
    client: &Client,
    key: &str,
    bucket: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let response = client.get_object().bucket(bucket).key(key).send().await?;

    let body = response
        .body
        .collect()
        .await
        .map_err(|e| format!("Failed to collect body: {}", e))?;

    Ok(body.to_vec())
}

// Add a helper function for text files
pub async fn get_object_as_string(
    client: &Client,
    key: &str,
    bucket: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let bytes = get_object(client, key, bucket).await?;

    String::from_utf8(bytes).map_err(|e| format!("Failed to convert body to string: {}", e).into())
}
