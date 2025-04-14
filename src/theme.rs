use iced::color;

/// Create a theme
///
/// We don't want to use `iced::theme::Theme` because we have zero control over names of the
/// fields.
macro_rules! theme {
    (
        $($key:ident = $value:expr),* $(,)?
    ) => {
        /// Theme of ferrishot
        struct Theme {
            $(
                $key: iced::Color,
            )*
        }
        /// Theme of ferrishot
        const THEME: Theme = Theme {
            $(
              $key: $value,
            )*
        };
    }
}

theme! {
    background = color!(0xff_ff_ff),
    foreground = color!(0x00_00_00),
}
