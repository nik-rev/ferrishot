//! Upload images to free services

use std::error::Error;

use documented::{Documented, DocumentedVariants};
use knus::DecodeScalar;
use serde::{Deserialize, Serialize};

#[derive(
    Copy,
    Clone,
    PartialEq,
    Debug,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    DocumentedVariants,
    Documented,
    DecodeScalar,
)]
#[serde(rename_all = "kebab-case")]
/// Choose which image upload service should be used by default when pressing "Upload Online"
pub enum ImageUploadService {
    /// Website: `https://0x0.st`
    TheNullPointer,
}

impl ImageUploadService {
    /// The base URL where image files should be uploaded
    fn post_url(self) -> &'static str {
        match self {
            ImageUploadService::TheNullPointer => "https://0x0.st",
        }
    }

    pub async fn upload_image(self, file_path: &std::path::Path) -> Result<String, Box<dyn Error>> {
        let request = crate::CLIENT
            .request(reqwest::Method::POST, self.post_url())
            .header(
                "User-Agent",
                format!("ferrishot/{:?}", env!("CARGO_PKG_VERSION")),
            );

        match self {
            ImageUploadService::TheNullPointer => Ok(request
                .multipart(
                    reqwest::multipart::Form::new()
                        .file("file", file_path)
                        .await?,
                )
                .send()
                .await?
                .text()
                .await?),
        }
    }
}
