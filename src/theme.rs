//! Theme for ferrishot
use core::fmt;

use iced::color;

/// For a given background color, what should the foreground color be
/// to make sure that it is readable?
///
/// Formula from <https://stackoverflow.com/a/3943023>
#[must_use]
pub fn foreground_for(color: iced::Color) -> iced::Color {
    let luma_coefficient = |x: f32| {
        if x <= 0.04045 {
            x / 12.92
        } else {
            ((x + 0.055) / 1.055).powf(2.4)
        }
    };

    let luminance = 0.0722f32.mul_add(
        luma_coefficient(color.b),
        0.2126f32.mul_add(
            luma_coefficient(color.r),
            0.7152 * luma_coefficient(color.g),
        ),
    );

    if luminance > 0.179 {
        iced::Color::BLACK
    } else {
        iced::Color::WHITE
    }
}

/// A wrapper for `iced::Color` to allow it to be used with `clap`
#[derive(Clone, Debug, Copy, PartialEq, Default)]
pub struct Color(pub iced::Color);

/// Parsing hex color failed
#[derive(Debug, thiserror::Error)]
pub enum HexColorParseError {
    /// Length is not correct
    #[error("Hex color must be 7 characters long")]
    InvalidLength,
    /// The hex string could not be parsed into 3 bytes
    #[error("Invalid hex color format")]
    InvalidFormat,
}

impl Color {
    /// Creates a `Color` from a hex string
    pub fn from_hex(hex: &str) -> Result<Self, HexColorParseError> {
        if hex.len() != 6 {
            return Err(HexColorParseError::InvalidLength);
        }
        match [0..=1, 2..=3, 4..=5].map(|i| hex.get(i).and_then(|c| u8::from_str_radix(c, 16).ok()))
        {
            [Some(r), Some(g), Some(b)] => Ok(Self(iced::Color::from_rgb(
                f32::from(r) / 255.0,
                f32::from(g) / 255.0,
                f32::from(b) / 255.0,
            ))),
            _ => Err(HexColorParseError::InvalidFormat),
        }
    }
    /// Update the opacity of this color.
    ///
    /// - `opacity = 1.0`: Opaque
    /// - `opacity = 0.0`: Transparent
    pub const fn with_opacity(mut self, opacity: f32) -> Self {
        self.0.a = opacity;
        self
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
            "#{:x}{:x}{:x} opacity={:.2}",
            (self.0.r * 255.0) as u8,
            (self.0.g * 255.0) as u8,
            (self.0.b * 255.0) as u8,
            self.0.a
        )
    }
}

impl From<iced::Color> for Color {
    fn from(value: iced::Color) -> Self {
        Self(value)
    }
}

/// Create a theme
///
/// We don't want to use `iced::theme::Theme` because we have zero control over names of the
/// fields.
macro_rules! theme {
    (
        $(
            $(#[$doc:meta])*
            $key:ident = $value:expr
        ),* $(,)?
    ) => {
        /// Theme of ferrishot
        pub struct Theme {
            $(
                $(#[$doc])*
                pub $key: iced::Color,
            )*
        }
        /// Theme of ferrishot
        pub static THEME: std::sync::LazyLock<Theme> = std::sync::LazyLock::new(|| Theme {
            $(
                $key: $value,
            )*
        });
    }
}

/// Default accent color.
const ACCENT_COLOR: iced::Color = color!(0x_ab_61_37);

theme! {
    /// Transparent: No color
    transparent = color!(0x_00_00_00, 0.0),
    /// First shadow to draw (stronger, but smaller)
    drop_shadow = color!(0x00_00_00, 0.5),
    /// Color of the background outside of the selection
    non_selected_region = color!(0x00_00_00, 0.5),
    /// Color of the background
    bg = color!(0x_00_00_00),
    /// Background color for errors
    error_bg = color!(0x_ff_00_00, 0.6),
    /// Color of text
    fg = color!(0x_ff_ff_ff),
    /// Color of text for the "width" and "height" size indicators
    size_indicator_fg = color!(0x_ff_ff_ff),
    /// Color of background text for the "width" and "height" size indicators
    size_indicator_bg = color!(0x_00_00_00, 0.5),
    /// Accent color, used for stuff like color of the frame + background
    /// of buttons
    accent = ACCENT_COLOR,
    /// Black or white text, depending on which one is more
    /// readable on a background that is `accent_bg`
    fg_on_accent_bg = foreground_for(ACCENT_COLOR),
    /// Background color of text selection
    text_selection_bg = ACCENT_COLOR.scale_alpha(0.3),
}
