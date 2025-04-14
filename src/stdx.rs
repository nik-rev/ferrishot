//! Extensions to `std`

use iced::Color;

/// For a given background color, what should the foreground color be
/// to make sure that it is readable?
///
/// Formula from <https://stackoverflow.com/a/3943023>
#[must_use]
pub fn foreground_for(color: Color) -> Color {
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
        Color::BLACK
    } else {
        Color::WHITE
    }
}
