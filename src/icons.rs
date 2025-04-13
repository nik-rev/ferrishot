//! Icons for ferrishot

use iced::{Color, Length, widget};

/// Helper to create a styled button with an icon
#[macro_export]
macro_rules! icon {
    ($icon:ident) => {{ $crate::icons::button($crate::icons::Icon::$icon) }};
}

/// Apply styles to the button
pub fn button<'a>(icon: crate::icons::Icon) -> iced::Element<'a, crate::message::Message> {
    use crate::icons;
    widget::button(
        widget::Svg::new(icon.svg())
            .style(|_, _| widget::svg::Style {
                color: Some(icons::ICON_COLOR),
            })
            .width(icons::ICON_SIZE)
            .height(icons::ICON_SIZE),
    )
    .width(icons::ICON_BUTTON_SIZE)
    .height(icons::ICON_BUTTON_SIZE)
    .style(|_, _| {
        let mut style = widget::button::Style::default().with_background(icons::ICON_BACKGROUND);
        style.border =
            iced::Border::default().rounded(iced::border::Radius::new(iced::Pixels::from(100)));
        style
    })
    .into()
}

/// width and height
pub const ICON_SIZE: Length = Length::Fixed(32.0);
/// Color used for the icons
pub const ICON_COLOR: Color = iced::color!(0xff_ff_ff);
/// Color to use for the background of icons
pub const ICON_BACKGROUND: Color = iced::color!(0x0f_0f_0f);
/// inner icon
pub const ICON_BUTTON_SIZE: Length = Length::Fixed(48.0);

/// Generates handles for macros and automatically includes all the icons
macro_rules! icons {
    (
        $(
            #[$doc:meta]
            $icon:ident
        ),* $(,)?
    ) => {
        /// Icons for ferrishot
        #[expect(dead_code, reason = "not all icons are used at the moment")]
        #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum Icon {
            $(
                #[$doc]
                $icon
            ),*
        }

        /// Private module so we don't leak implementation detail of the static icons
        mod __static_icons {
            $(
                #[expect(nonstandard_style, reason = "handy for creating statics")]
                pub(super) static $icon: std::sync::LazyLock<iced::widget::svg::Handle> = std::sync::LazyLock::new(|| {
                    iced::widget::svg::Handle::from_memory(include_bytes!(concat!(
                        "../icons/",
                        stringify!($icon),
                        ".svg"
                    )))
                });
            )*

        }

        impl Icon {
            /// Obtain this icon's svg handle
            pub fn svg(self) -> iced::widget::svg::Handle {
                match self {
                    $(Self::$icon => __static_icons::$icon.clone()),*
                }
            }
        }
    }
}

icons! {
    /// Save the image to a path by opening the file dialog
    Save,
    /// Drawing a circle
    Circle,
    /// Copy the image to clipboard
    Clipboard,
    /// Close the app
    Close,
    /// Switch to Cursor tool, allows resizing and dragging the selection around
    Cursor,
    /// Select the entire image
    Fullscreen,
    /// Draw on the image
    Pen,
    /// Draw a square
    Square,
    /// Add text
    Text,
}
