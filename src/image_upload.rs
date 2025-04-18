//! Upload images to free services

use std::{error::Error, sync::LazyLock};

use reqwest::Client;

use serde::Deserialize;

/// Generate a single client for usage in the app
static CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

/// Upload image to a service
pub trait ImageUploadService {
    /// The base URL where image files should be uploaded
    const POST_URL: &'static str;

    /// Upload a file to the specified service, returning the URL to the file
    async fn upload(file_path: &std::path::Path) -> Result<String, Box<dyn Error>>;

    /// Start a request for this URL
    fn request() -> reqwest::RequestBuilder {
        CLIENT
            .request(reqwest::Method::POST, Self::POST_URL)
            .header(
                "User-Agent",
                format!("ferrishot/{:?}", env!("CARGO_PKG_VERSION")),
            )
    }
}

/// https://0x0.st
#[derive(Deserialize)]
pub struct ZeroZero {
    /// Url of the uploaded link
    file_url: String,
}

impl ImageUploadService for ZeroZero {
    const POST_URL: &'static str = "https://0x0.st";

    async fn upload(file_path: &std::path::Path) -> Result<String, Box<dyn Error>> {
        let form = reqwest::multipart::Form::new()
            .file("file", file_path)
            .await?;

        let client = Self::request().multipart(form).send().await?;
        Ok(client.text().await?)
    }
}
