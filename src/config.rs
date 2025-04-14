//! Command line arguments to configure the program
use core::fmt;
use std::sync::LazyLock;

use clap::Parser;

/// A wrapper for `iced::Color` to allow it to be used with `clap`
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Color(iced::Color);

/// Parsing hex color failed
#[derive(Debug, thiserror::Error)]
pub enum HexColorParseError {
    /// Missing # character at the beginning
    #[error("Hex color must start with a `#`")]
    MissingHex,
    /// Length is not correct
    #[error("Hex color must be 7 characters long")]
    InvalidLength,
    /// The hex string could not be parsed into 3 bytes
    #[error("Invalid hex color format")]
    InvalidFormat,
}

/// Configuration of the app
pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::parse);

impl Color {
    /// Creates a `Color` from a hex string
    pub fn from_hex(hex: &str) -> Result<Self, HexColorParseError> {
        if !hex.starts_with('#') {
            return Err(HexColorParseError::MissingHex);
        }
        if hex.len() != 7 {
            return Err(HexColorParseError::InvalidLength);
        }
        match [1..=2, 3..=4, 5..=6].map(|i| hex.get(i).and_then(|c| u8::from_str_radix(c, 16).ok()))
        {
            [Some(r), Some(g), Some(b)] => Ok(Self(iced::Color::from_rgb(
                f32::from(r) / 255.0,
                f32::from(g) / 255.0,
                f32::from(b) / 255.0,
            ))),
            _ => Err(HexColorParseError::InvalidFormat),
        }
    }
}

impl std::str::FromStr for Color {
    type Err = HexColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s)
    }
}

impl fmt::Display for Color {
    #[expect(clippy::cast_sign_loss, reason = "guaranteed to be positive")]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "float in int range 0..=255 can be fully represented"
    )]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#{:x}{:x}{:x}",
            (self.0.r * 255.0) as u8,
            (self.0.g * 255.0) as u8,
            (self.0.b * 255.0) as u8
        )
    }
}

impl From<Color> for iced::Color {
    fn from(value: Color) -> Self {
        value.0
    }
}

impl From<iced::Color> for Color {
    fn from(value: iced::Color) -> Self {
        Self(value)
    }
}

/// Configuration for the program
#[derive(Parser, Debug)]
#[command(version, about, author = "Nik Revenco")]
pub struct Config {
    /// The first selection will be copied to the clipboard as soon as the left mouse button is released
    #[arg(long)]
    pub instant: bool,
    /// Color for elements in the UI, such as the selection border and buttons
    #[arg(long, value_name = "HEX_COLOR", default_value_t = iced::color!(0xab_61_37).into())]
    pub accent_color: Color,
}
