//! Configurable constant knobs
use iced::Color;
use std::time::Duration;

/// When there is an error, display it for this amount of time
pub const ERROR_TIMEOUT: Duration = Duration::from_secs(5);
/// Radius of each of the 4 corner circles in the frame drawn around the selection
pub const FRAME_CIRCLE_RADIUS: f32 = 6.0;
/// Color of the selection stroke and corners (the frame)
pub const FRAME_COLOR: Color = Color::WHITE;
/// The area around each side of the frame which allows that side to be hovered over and resized
pub const FRAME_INTERACTION_AREA: f32 = 30.0;
/// The size of the lines of the frame of the selection
pub const FRAME_WIDTH: f32 = 2.0;
/// The color of the background for non-selected regions
pub const NON_SELECTED_REGION_COLOR: Color = iced::color!(0x00_00_00, 0.15);

/// Color used for the icons
pub const ICON_COLOR: Color = iced::color!(0xff_ff_ff);
/// Color to use for the background of icons
pub const ICON_BACKGROUND: Color = iced::color!(0x0f_0f_0f);
/// Width and height for icons *inside* of buttons
pub const ICON_SIZE: f32 = 32.0;
/// Size of the button for the icon, which includes the
/// icon itself and space around it (bigger than `ICON_SIZE`)
pub const ICON_BUTTON_SIZE: f32 = 48.0;
/// padding between icons
pub const SPACE_BETWEEN_ICONS: f32 = 2.0;
