use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScreenshotError {
    #[error("Unable to get the mouse position")]
    NoMousePosition,
    #[error("Could not get the monitor: {0}")]
    NoMonitor(xcap::XCapError),
    #[error("Could not get the monitor: {0}")]
    NoScreenshot(xcap::XCapError),
}

/// Take a screenshot and return a handle to the image
pub fn screenshot() -> Result<iced::widget::image::Handle, ScreenshotError> {
    let mouse_position::mouse_position::Mouse::Position { x, y } =
        mouse_position::mouse_position::Mouse::get_mouse_position()
    else {
        return Err(ScreenshotError::NoMousePosition);
    };

    let monitor = xcap::Monitor::from_point(x, y).map_err(ScreenshotError::NoMonitor)?;

    let screenshot = monitor
        .capture_image()
        .map_err(ScreenshotError::NoScreenshot)?;

    Ok(iced::widget::image::Handle::from_rgba(
        screenshot.width(),
        screenshot.height(),
        screenshot.into_raw(),
    ))
}
