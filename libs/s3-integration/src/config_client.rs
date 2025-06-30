use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;

pub async fn config_client() -> Client {
    let aws_config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    Client::new(&aws_config)
}
