//! The ferrishot app

mod app;
mod background_image;
mod clipboard;
mod config;
mod corners;
mod logging;
mod message;
mod mouse;
mod rectangle;
mod screenshot;
mod selection;

/// Save the image to a path by opening the file dialog
#[expect(dead_code, reason = "for later")]
const SAVE_ICON: &[u8; 616] = include_bytes!("../icons/save.svg");
/// Drawing a circle
const CIRCLE_ICON: &[u8; 272] = include_bytes!("../icons/circle.svg");
/// Copy the image to clipboard
#[expect(dead_code, reason = "for later")]
const CLIPBOARD_ICON: &[u8; 530] = include_bytes!("../icons/clipboard.svg");
/// Close the app
#[expect(dead_code, reason = "for later")]
const CLOSE_ICON: &[u8; 304] = include_bytes!("../icons/close.svg");
/// Switch to Cursor tool, allows resizing and dragging the selection around
#[expect(dead_code, reason = "for later")]
const CURSOR_ICON: &[u8; 1014] = include_bytes!("../icons/cursor.svg");
/// Select the entire image
#[expect(dead_code, reason = "for later")]
const FULLSCREEN_ICON: &[u8; 2090] = include_bytes!("../icons/fullscreen.svg");
/// Draw on the image
#[expect(dead_code, reason = "for later")]
const PEN_ICON: &[u8; 419] = include_bytes!("../icons/pen.svg");
/// Draw a square
#[expect(dead_code, reason = "for later")]
const SQUARE_ICON: &[u8; 277] = include_bytes!("../icons/square.svg");
/// Add text
#[expect(dead_code, reason = "for later")]
const TEXT_ICON: &[u8; 266] = include_bytes!("../icons/text.svg");

/// When there is an error, display it for this amount of time
const ERROR_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);
/// Radius of the 4 corners of the selection
const CORNER_RADIUS: f32 = 6.0;
/// Color of the selection stroke and corners
const SELECTION_COLOR: Color = Color::WHITE;
/// The area around each side which allows that side to be hovered over and
/// resized
const INTERACTION_AREA: f32 = 30.0;
/// The size of the border of the square
const STROKE_SIZE: f32 = 2.0;
/// The color of the background for non-selected regions
const SHADE_COLOR: Color = iced::color!(0x00_00_00, 0.15);
/// width and height
const ICON_SIZE: Length = Length::Fixed(32.0);
/// Color used for the icons
const ICON_COLOR: Color = iced::color!(0xff_ff_ff);
/// Color to use for the background of icons
const ICON_BACKGROUND: Color = iced::color!(0x0f_0f_0f);
/// inner icon
const ICON_BUTTON_SIZE: Length = Length::Fixed(48.0);

#[cfg(target_os = "linux")]
pub use clipboard::{CLIPBOARD_DAEMON_ID, run_clipboard_daemon};

pub use app::App;
pub use app::SAVED_IMAGE;
use iced::Color;
use iced::Length;
pub use logging::initialize_logging;
