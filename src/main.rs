use aws_sdk_s3::{
    config::{Credentials, Region, RequestChecksumCalculation, ResponseChecksumValidation},
    primitives::ByteStream,
};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    // load .env file
    dotenv().expect(".env file not found.");
    // insert a environment variable
    let endpoint_url: String =
        env::var("ENDPOINT_URL").expect("ENDPOINT_URL not found in .env file.");
    let access_key_id: String =
        env::var("ACCESS_KEY_ID").expect("ACCESS_KEY_ID not found in .env file.");
    let secret_access_key: String =
        env::var("SECRET_ACCESS_KEY").expect("SECRET_ACCESS_KEY not found in .env file.");

    let credentials = Credentials::new(access_key_id, secret_access_key, None, None, "");

    let r2_config = aws_sdk_s3::config::Builder::new()
        .credentials_provider(credentials)
        .region(Region::new("auto"))
        .endpoint_url(endpoint_url)
        .set_request_checksum_calculation(Some(RequestChecksumCalculation::WhenRequired))
        .set_response_checksum_validation(Some(ResponseChecksumValidation::WhenRequired))
        .clone()
        .build();

    let client = aws_sdk_s3::Client::from_conf(r2_config);

    //put object
    let bucket_name: String = env::var("BUCKET_NAME").expect("BUCKET_NAME not found in .env file.");
    client
        .put_object()
        .bucket(&bucket_name)
        .key("test.txt")
        .content_type("text/plain")
        .body(ByteStream::from("Hello, World!".as_bytes().to_vec()))
        .send()
        .await
        .unwrap();

    //get object
    let response = client
        .get_object()
        .bucket(&bucket_name)
        .key("test.txt")
        .send()
        .await
        .unwrap();
    println!(
        "Body: {:?}",
        response.body.collect().await.unwrap().into_bytes()
    );

    //delete object
    client
        .delete_object()
        .bucket(&bucket_name)
        .key("test.txt")
        .send()
        .await
        .unwrap();
}
