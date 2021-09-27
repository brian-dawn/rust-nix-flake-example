use aws_sdk_s3::Client;
use std::error::Error;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let resp = client
        .list_objects_v2()
        .bucket("bucket-name")
        .send()
        .await?;

    let contents = resp.contents.unwrap_or_default();
    let size = contents.len();
    for object in contents {
        let object_key = object.key;
        println!("{}", object_key.unwrap_or("nope".to_owned()));
    }

    Ok(())
}
