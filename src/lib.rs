//! The groxshot utility functions

mod app;
mod background_image;
mod corners;
mod message;
mod mouse;
mod rectangle;
mod screenshot;
mod selection;

/// Radius of the 4 corners of the selection
pub const CORNER_RADIUS: f32 = 6.;
/// Color of the selection stroke and corners
pub const SELECTION_COLOR: iced::Color = iced::Color::WHITE;
/// The area around each side which allows that side to be hovered over and
/// resized
pub const INTERACTION_AREA: f32 = 20.;
/// The size of the border of the square
pub const STROKE_SIZE: f32 = 2.;

pub use app::App;
