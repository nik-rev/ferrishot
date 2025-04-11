//! Takes screenshot of the desktop

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
    #[error("Couldn not take a screenshot: {0}")]
    Screenshot(xcap::XCapError),
}

/// Take a screenshot and return a handle to the image
pub fn screenshot() -> Result<iced::widget::image::Handle, ScreenshotError> {
    let mouse_position::mouse_position::Mouse::Position { x, y } =
        mouse_position::mouse_position::Mouse::get_mouse_position()
    else {
        return Err(ScreenshotError::MousePosition);
    };

    let monitor = xcap::Monitor::from_point(x, y).map_err(ScreenshotError::Monitor)?;

    let screenshot = monitor
        .capture_image()
        .map_err(ScreenshotError::Screenshot)?;

    Ok(iced::widget::image::Handle::from_rgba(
        screenshot.width(),
        screenshot.height(),
        screenshot.into_raw(),
    ))
}
