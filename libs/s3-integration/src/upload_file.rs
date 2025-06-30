use aws_sdk_s3::{error::SdkError, operation::put_object::PutObjectError, Client};

pub async fn upload_file(
    client: &Client,
    bucket: &str,
    folder: &str,
    file_name: &str,
    file_path: &str,
) -> Result<(), SdkError<PutObjectError>> {
    let body = aws_sdk_s3::primitives::ByteStream::from_path(file_path)
        .await
        .expect("Failed to get body");
    client
        .put_object()
        .bucket(bucket)
        .key(format!("{}/{}", folder, file_name))
        .body(body)
        .send()
        .await
        .expect(format!("Failed to upload file: {}", file_name).as_str());
    Ok(())
}
