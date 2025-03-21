use aws_config::Region;
use aws_sdk_s3::{
    config::{self, Credentials},
    Client,
};

pub async fn create_client(
    access_key: &str,
    secret_key: &str,
    endpoint_url: &str,
    region: &str,
) -> Client {
    let config = config::Builder::new()
        .region(Region::new(region.to_string()))
        .credentials_provider(Credentials::new(
            access_key.to_string(),
            secret_key.to_string(),
            None,
            None,
            "",
        ))
        .endpoint_url(endpoint_url.to_string())
        .force_path_style(true)
        .build();

    Client::from_conf(config)
}
