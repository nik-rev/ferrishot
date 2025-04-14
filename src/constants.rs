//! Configurable constant knobs
use iced::{Color, Shadow, color, theme::Palette};
use std::time::Duration;

/// Ferrishot's custom theme
pub const PALETTE: Palette = Palette {
    background: Color::WHITE,
    text: Color::BLACK,
    primary: color!(0xab_61_37),
    success: color!(0x12_66_4f),
    warning: color!(0xff_c1_4e),
    danger: color!(0xc3_42_3f),
};

/// When there is an error, display it for this amount of time
pub const ERROR_TIMEOUT: Duration = Duration::from_secs(5);
/// Radius of each of the 4 corner circles in the frame drawn around the selection
pub const FRAME_CIRCLE_RADIUS: f32 = 6.0;
/// First shadow to draw (stronger, but smaller)
pub const DROP_SHADOW_COLOR: Color = color!(0x00_00_00, 0.5);
/// Shadow to apply to elements
pub const DROP_SHADOW: Shadow = Shadow {
    color: DROP_SHADOW_COLOR,
    blur_radius: 3.0,
    offset: iced::Vector { x: 0.0, y: 0.0 },
};
/// The area around each side of the frame which allows that side to be hovered over and resized
pub const FRAME_INTERACTION_AREA: f32 = 30.0;
/// The size of the lines of the frame of the selection
pub const FRAME_WIDTH: f32 = 2.0;
/// The color of the background for non-selected regions
pub const NON_SELECTED_REGION_COLOR: Color = color!(0x00_00_00, 0.4);

/// Width and height for icons *inside* of buttons
pub const ICON_SIZE: f32 = 32.0;
/// Size of the button for the icon, which includes the
/// icon itself and space around it (bigger than `ICON_SIZE`)
pub const ICON_BUTTON_SIZE: f32 = 37.0;
/// padding between icons
pub const SPACE_BETWEEN_ICONS: f32 = 2.0;
