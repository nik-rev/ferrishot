//! Configuration of ferrishot
use std::{fs, path::PathBuf, sync::LazyLock};

use clap::Parser;
use etcetera::BaseStrategy;
use miette::IntoDiagnostic as _;

use crate::{
    corners::{Direction, RectPlace},
    key::{KeyMods, KeySequence},
    message::Message,
};

/// Command line arguments for the program
#[derive(Parser, Debug)]
#[command(version, about, author = "Nik Revenco")]
pub struct Cli {
    /// Specifies the config file to use
    #[arg(
        long,
        value_name = "file.kdl",
        default_value_t = etcetera::choose_base_strategy()
            .map(|strategy| {
                strategy
                    .config_dir()
                    .join("ferrishot")
                    .join("config.kdl")
                    .to_string_lossy()
                    .to_string()
            })
            .unwrap_or("ferrishot.kdl".to_owned())
    )]
    pub config_file: String,
}

/// `Some(...)` if `$(...)?` exists, `None` otherwise
#[macro_export]
macro_rules! opt {
    () => {
        None::<&'static str>
    };
    ($any:tt) => {
        Some($any)
    };
}

/// Declare default keybindings
#[macro_export]
macro_rules! default_keys {
    (
        $(
            $key:literal $(mods=$modifier:literal)? => $message:expr
        ),* $(,)?
    ) => {{
        let mut map = std::collections::HashMap::new();
        $(
            map.insert(
                $key.parse::<$crate::key::KeySequence>().expect(concat!("default keybinding ", $key, " is incorrect")),
                ($crate::opt!($($modifier)?).unwrap_or_default().parse::<$crate::key::KeyMods>().expect(concat!("modifiers ", $(stringify!($modifier), )? "is incorrect")), $message)
            );
        )*
        $crate::key::KeyMap::new(map)
    }};
}

/// Utility macro to create a theme with default colors
///
/// Implementing the `Default` trait would be a lot of repetition and it cannot be automatically
/// derived using the values in `#[knus(default = ...)]`.
#[macro_export]
macro_rules! theme {
    (
        $(
            #[$($doc:meta)*]
            $key:ident = $default_color:expr
        ),* $(,)?
    ) => {
        /// Theme for ferrishot
        #[derive(knus::Decode, Debug)]
        pub struct Theme {
            $(
                #[$($doc),*]
                #[knus(default = $crate::theme::Color($default_color), child, unwrap(argument, str))]
                pub $key: $crate::theme::Color,
            )*
        }

        impl Default for Theme {
            fn default() -> Self {
                Self {
                    $(
                        $key: $crate::theme::Color($default_color),
                    )*
                }
            }
        }
    };
}

/// Utility macro to create two similar config structs
/// The only thing that is different between `KdlConfig` and `Config`
/// is that the `keys` field changes.
///
/// It also implements `Default`, removing quite a lot of boilerplate.
/// We would have to specify `#[knus(default(...))]` and `impl Default`.
#[macro_export]
macro_rules! config {
    (
        $(
            $(#[$doc:meta])*
            $key:ident: $typ:ty = $default:expr
        ),* $(,)?
    ) => {
        #[derive(knus::Decode, Debug)]
        /// Raw config, not processed into a more useful structure yet
        pub struct KdlConfig {
            $(
                $(#[$doc])*
                #[knus(default = $default, child, unwrap(argument))]
                pub $key: $typ,
            )*
            /// Theme
            #[knus(default, child)]
            pub theme: Theme,
            /// Keybindings
            #[knus(default, child)]
            pub keys: $crate::config::Keys,
        }

        impl Default for KdlConfig {
            fn default() -> Self {
                Self {
                    theme: Theme::default(),
                    keys: $crate::config::Keys::default(),
                    $(
                        $key: $default
                    ),*
                }
            }
        }

        /// Processed config, with keybindings that are ready to be used
        #[derive(Debug)]
        pub struct Config {
            $(
                $(#[$doc])*
                pub $key: $typ,
            )*
            /// Theme
            pub theme: Theme,
            /// A list of processed keybindings for ferrishot.
            pub keys: $crate::key::KeyMap,
        }

        impl Default for Config {
            fn default() -> Self {
                Self {
                    theme: Theme::default(),
                    keys: $crate::key::KeyMap::default(),
                    $(
                        $key: $default
                    ),*
                }
            }
        }
    }
}

/// Configuration of the app
///
/// Static as that means it will never change once the app is launched.
/// It also makes it easy to get the config values anywhere from the app, even where we don't have access to
/// the `App`.
pub static CONFIG: LazyLock<crate::defaults::Config> = LazyLock::new(|| {
    let cli = Cli::parse();

    let kdl_config = (|| -> miette::Result<crate::defaults::KdlConfig> {
        let config_file = cli.config_file.as_str();
        let config_file_path = PathBuf::from(config_file);

        // if there is no config file, act as if it's simply empty
        let config = knus::parse::<crate::defaults::KdlConfig>(
            cli.config_file,
            &fs::read_to_string(&config_file_path)
                .into_diagnostic()
                .unwrap_or_default(),
        )?;

        Ok(config)
    })();

    match kdl_config {
        Ok(kdl_config) => {
            let kdl_keys = kdl_config.keys.keys;
            let mut keys = crate::defaults::keymap();
            for key in kdl_keys {
                match key {
                    Key::CopyToClipboard(key_sequence, key_mods) => {
                        keys.insert(key_sequence, key_mods, Message::CopyToClipboard);
                    }
                    Key::SaveScreenshot(key_sequence, key_mods) => {
                        keys.insert(key_sequence, key_mods, Message::SaveScreenshot);
                    }
                    Key::Exit(key_sequence, key_mods) => {
                        keys.insert(key_sequence, key_mods, Message::Exit);
                    }
                    Key::Goto(rect_place, key_sequence, key_mods) => {
                        keys.insert(key_sequence, key_mods, Message::Goto(rect_place));
                    }
                    Key::Move(direction, amount, key_sequence, key_mods) => {
                        keys.insert(
                            key_sequence,
                            key_mods,
                            Message::Move(direction, amount * kdl_config.movement_multiplier),
                        );
                    }
                    Key::Extend(direction, amount, key_sequence, key_mods) => {
                        keys.insert(
                            key_sequence,
                            key_mods,
                            Message::Extend(direction, amount * kdl_config.movement_multiplier),
                        );
                    }
                    Key::Shrink(direction, amount, key_sequence, key_mods) => {
                        keys.insert(
                            key_sequence,
                            key_mods,
                            Message::Shrink(direction, amount * kdl_config.movement_multiplier),
                        );
                    }
                }
            }

            crate::defaults::Config {
                theme: kdl_config.theme,
                keys,
                instant: kdl_config.instant,
                default_image_upload_provider: kdl_config.default_image_upload_provider,
                size_indicator: kdl_config.size_indicator,
                movement_multiplier: kdl_config.movement_multiplier,
            }
        }
        Err(miette_error) => {
            eprintln!("{miette_error:?}");
            std::process::exit(1);
        }
    }
});

/// Keybindings for ferrishot
#[derive(knus::Decode, Debug, Default)]
pub struct Keys {
    /// A list of raw keybindings for ferrishot, directly as read from the config file
    #[knus(children)]
    pub keys: Vec<Key>,
}

/// A list of keybindings which exist in the app
#[derive(knus::Decode, Debug)]
pub enum Key {
    /// Copy the selected region as a screenshot to the clipboard
    CopyToClipboard(
        #[knus(property(name = "key"), str)] KeySequence,
        #[knus(default, property(name = "mods"), str)] KeyMods,
    ),
    /// Save the screenshot as a path
    SaveScreenshot(
        #[knus(property(name = "key"), str)] KeySequence,
        #[knus(default, property(name = "mods"), str)] KeyMods,
    ),
    /// Exit the application
    Exit(
        #[knus(property(name = "key"), str)] KeySequence,
        #[knus(default, property(name = "mods"), str)] KeyMods,
    ),
    /// Teleport the selection to the given area
    Goto(
        // where to move the rect
        #[knus(argument, str)] RectPlace,
        #[knus(property(name = "key"), str)] KeySequence,
        #[knus(default, property(name = "mods"), str)] KeyMods,
    ),
    /// Shift the selection in the given direction by pixels
    Move(
        #[knus(argument)] Direction,
        // strength
        #[knus(argument)] u32,
        #[knus(property(name = "key"), str)] KeySequence,
        #[knus(default, property(name = "mods"), str)] KeyMods,
    ),
    /// Increase the size of the selection in the given direction by pixels
    Extend(
        // where to extend
        #[knus(argument)] Direction,
        // strength
        #[knus(argument)] u32,
        // binding
        #[knus(property(name = "key"), str)] KeySequence,
        #[knus(default, property(name = "mods"), str)] KeyMods,
    ),
    /// Decrease the size of the selection in the given direction by pixels
    Shrink(
        // shrink in this direction
        #[knus(argument)] Direction,
        // strength
        #[knus(argument)] u32,
        // binding
        #[knus(property(name = "key"), str)] KeySequence,
        #[knus(default, property(name = "mods"), str)] KeyMods,
    ),
}
