//! Icons for ferrishot
//!
//! Icons are stored in the `icons/` directory.
//! Icons are declared at the invocation of the `icons!` macro.
//! Each `Icon` must have a corresponding `icons/Icon.svg` file.

/// Helper to create a styled button with an icon
#[macro_export]
macro_rules! icon {
    ($icon:ident) => {{ $crate::widgets::icon($crate::icons::Icon::$icon) }};
}

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
