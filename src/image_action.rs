//! One of 3 actions:
//!
//! - Upload image
//! - Copy image
//! - Save image
use std::path::PathBuf;

use iced::Task;
use iced::widget as w;
use image::DynamicImage;

use crate::image_upload::ImageUploaded;
use crate::{App, config::KeyAction, rect::RectangleExt as _, ui::popup::image_uploaded};

/// Action to take with the image
#[derive(clap::ValueEnum, Debug, Clone, Copy, strum::EnumIs)]
pub enum Message {
    /// Copy image to the clipboard
    Copy,
    /// Save image to a file
    Save,
    /// Upload image to the internet
    Upload,
}

/// Data about the image
pub struct ImageData {
    /// Height of the image (pixels)
    height: u32,
    /// Width of the image (pixels)
    width: u32,
}

/// The output of an image action
pub enum Output {
    /// Copied to the clipboard
    Copied,
    /// Saved to a path
    Saved,
    /// Uploaded to the internet
    Uploaded {
        /// information about the uploaded image
        data: ImageUploaded,
        /// file size in bytes
        file_size: u64,
        /// Path to the uploaded image
        path: PathBuf,
    },
}

impl Message {
    /// Convert this into a key action
    pub fn into_key_action(self) -> KeyAction {
        match self {
            Self::Copy => KeyAction::CopyToClipboard,
            Self::Save => KeyAction::SaveScreenshot,
            Self::Upload => KeyAction::UploadScreenshot,
        }
    }

    /// Execute the action
    pub async fn execute(self, image: DynamicImage) -> Result<(Output, ImageData), String> {
        let image_data = ImageData {
            height: image.height(),
            width: image.width(),
        };

        match self {
            Self::Copy => {
                let clipboard = arboard::ImageData {
                    width: image.width() as usize,
                    height: image.height() as usize,
                    bytes: std::borrow::Cow::Borrowed(image.as_bytes()),
                };

                crate::clipboard::set_image(clipboard)
                    .map(|_| (Output::Copied, image_data))
                    .map_err(|e| format!("Could not copy the image: {e}"))
            }
            Self::Save => {
                let _ = crate::SAVED_IMAGE.set(image);
                Ok((Output::Saved, image_data))
            }
            Self::Upload => {
                let path = tempfile::TempDir::new()
                    .map_err(|e| e.to_string())?
                    .into_path()
                    .join("ferrishot-screenshot.png");

                // TODO: allow configuring the upload format
                image
                    .save_with_format(&path, image::ImageFormat::Png)
                    .map_err(|e| e.to_string())?;

                let file_size = path.metadata().map(|m| m.len()).unwrap_or(0);

                let x = crate::image_upload::upload(&path)
                    .await
                    .map_err(|err| err.into_iter().next().expect("at least 1 upload provider"))?;

                Ok((
                    Output::Uploaded {
                        path,
                        data: x,
                        file_size,
                    },
                    image_data,
                ))
            }
        }
    }
}

impl crate::message::Handler for Message {
    fn handle(self, app: &mut App) -> Task<crate::Message> {
        let Some(rect) = app.selection.map(|sel| sel.rect.norm()) else {
            app.errors.push(match self {
                Self::Copy => "There is no selection to copy",
                Self::Upload => "There is no selection to upload",
                Self::Save => "There is no selection to save",
            });
            return Task::none();
        };

        let image = App::process_image(rect, &app.image);

        if self.is_upload() {
            app.is_uploading_image = true;
        }

        Task::future(async move {
            match self.execute(image).await {
                Ok((Output::Saved | Output::Copied, _)) => crate::message::Message::Exit,
                Ok((
                    Output::Uploaded {
                        path,
                        data,
                        file_size,
                    },
                    ImageData { height, width },
                )) => crate::Message::ImageUploaded(image_uploaded::Message::ImageUploaded(
                    image_uploaded::ImageUploadedData {
                        image_uploaded: data,
                        uploaded_image: w::image::Handle::from_path(&path),
                        height,
                        width,
                        file_size,
                    },
                )),
                Err(err) => crate::Message::Error(err),
            }
        })
    }
}

/// The image to save to a file, chosen by the user in a file picker.
///
/// Unfortunately, there is simply no way to communicate something from
/// the inside of an iced application to the outside: i.e. "Return" something
/// from an iced program exiting. So we have to use a global variable for this.
///
/// This global is mutated just *once* at the end of the application's lifetime,
/// when the window closes.
///
/// It is then accessed just *once* to open the file dialog and let the user pick
/// where they want to save their image.
///
/// Yes, at the moment we want this when using Ctrl + S to save as file:
/// 1. Close the application to save the file and generate the image we'll save
/// 2. Open the file explorer, and save the image to the specified path
///
/// When the file explorer is spawned from the inside of an iced window, closing
/// this window will then also close the file explorer. It means that we can't
/// close the window and then spawn an explorer.
///
/// The other option is to have both windows open at the same time. But this
/// would be really odd. First of all, we will need to un-fullscreen the App
/// because the file explorer can spawn under the app.
///
/// This is going to be sub-optimal. Currently, we give off the illusion of
/// drawing shapes and rectangles on top of the desktop. It is not immediately
/// obvious that the app is just another window which is full-screen.
/// Doing the above would break that illusion.
///
/// Ideally, we would be able to spawn a file explorer *above* the window without
/// having to close this. But this seems to not be possible. Perhaps in the
/// future there will be some kind of file explorer Iced widget that we
/// can use instead of the native file explorer.
pub static SAVED_IMAGE: std::sync::OnceLock<image::DynamicImage> = std::sync::OnceLock::new();
