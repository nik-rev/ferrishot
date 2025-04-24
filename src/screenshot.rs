//! Takes screenshot of the desktop

use iced::{advanced::image::Bytes, widget::image::Handle};

/// A handle pointing to decoded image pixels in RGBA format.
///
/// This is a more specialized version of `iced::widget::image::Handle`
#[derive(Debug, Clone)]
pub struct RgbaHandle(Handle);

impl RgbaHandle {
    /// Create handle to an image represented in RGBA format
    pub fn new(width: u32, height: u32, pixels: impl Into<Bytes>) -> Self {
        Self(Handle::from_rgba(width, height, pixels.into()))
    }

    /// Width of the image
    pub fn width(&self) -> u32 {
        self.raw().0
    }

    /// Height of the image
    pub fn height(&self) -> u32 {
        self.raw().1
    }

    /// RGBA bytes of the image
    pub fn bytes(&self) -> &Bytes {
        self.raw().2
    }

    /// Returns the width, height and RGBA pixels
    fn raw(&self) -> (u32, u32, &Bytes) {
        let Handle::Rgba {
            width,
            height,
            ref pixels,
            ..
        } = self.0
        else {
            unreachable!("handle is guaranteed to be Rgba")
        };
        (width, height, pixels)
    }
}

impl From<RgbaHandle> for Handle {
    fn from(value: RgbaHandle) -> Self {
        value.0
    }
}

/// Could not retrieve the screenshot
#[derive(thiserror::Error, Debug)]
pub enum ScreenshotError {
    /// The position of the mouse is unavailable
    #[error("Could not get position of the mouse")]
    MousePosition,
    #[error("Could not get the active monitor: {0}")]
    /// There is no active monitor
    Monitor(xcap::XCapError),
    /// Could not capture the screenshot for some reason
    #[error("Could not take a screenshot: {0}")]
    Screenshot(xcap::XCapError),
}

/// Take a screenshot and return a handle to the image
pub fn screenshot() -> Result<RgbaHandle, ScreenshotError> {
    let mouse_position::mouse_position::Mouse::Position { x, y } =
        mouse_position::mouse_position::Mouse::get_mouse_position()
    else {
        return Err(ScreenshotError::MousePosition);
    };

    let monitor = xcap::Monitor::from_point(x, y).map_err(ScreenshotError::Monitor)?;

    let screenshot = monitor
        .capture_image()
        .map_err(ScreenshotError::Screenshot)?;

    Ok(RgbaHandle::new(
        screenshot.width(),
        screenshot.height(),
        screenshot.into_raw(),
    ))
}
