//! Configurable constant knobs
use std::time::Duration;

/// When there is an error, display it for this amount of time
pub const ERROR_TIMEOUT: Duration = Duration::from_secs(5);
/// Radius of each of the 4 corner circles in the frame drawn around the selection
pub const FRAME_CIRCLE_RADIUS: f32 = 6.0;
/// Shadow to apply to elements
/// The area around each side of the frame which allows that side to be hovered over and resized
pub const FRAME_INTERACTION_AREA: f32 = 30.0;
/// The size of the lines of the frame of the selection
pub const FRAME_WIDTH: f32 = 2.0;

/// Width and height for icons *inside* of buttons
pub const ICON_SIZE: f32 = 32.0;
/// Size of the button for the icon, which includes the
/// icon itself and space around it (bigger than `ICON_SIZE`)
pub const ICON_BUTTON_SIZE: f32 = 37.0;
/// padding between icons
pub const SPACE_BETWEEN_ICONS: f32 = 2.0;
