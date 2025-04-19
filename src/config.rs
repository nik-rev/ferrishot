//! Configuration of ferrishot
use std::{collections::HashMap, fs, path::PathBuf, sync::LazyLock};

use clap::Parser;
use etcetera::BaseStrategy;
use miette::IntoDiagnostic as _;

use crate::{
    corners::{Direction, RectPlace},
    image_upload::ImageUploadService,
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

/// Configuration of the app
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    use iced::keyboard::Key as IcedKey;
    use iced::keyboard::key::Named as IcedNamed;

    let cli = Cli::parse();
    let kdl_config = (|| -> miette::Result<KdlConfig> {
        let config_file = cli.config_file.as_str();
        let config_file_path = PathBuf::from(config_file);

        // if there is no config file, act as if it's simply empty
        let config = knus::parse::<KdlConfig>(
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
            // default keybindings
            let mut keys = HashMap::from([(
                KeySequence((IcedKey::Named(IcedNamed::Escape), None)),
                (KeyMods::default(), Message::Exit),
            )]);
            for key in kdl_keys {
                match key {
                    Key::CopyToClipboard(key_sequence, key_mods) => {
                        keys.insert(key_sequence, (key_mods, Message::CopyToClipboard));
                    }
                    Key::SaveScreenshot(key_sequence, key_mods) => {
                        keys.insert(key_sequence, (key_mods, Message::SaveScreenshot));
                    }
                    Key::Exit(key_sequence, key_mods) => {
                        keys.insert(key_sequence, (key_mods, Message::Exit));
                    }
                    Key::Goto(rect_place, key_sequence, key_mods) => {
                        keys.insert(key_sequence, (key_mods, Message::Goto(rect_place)));
                    }
                    Key::Move(direction, amount, key_sequence, key_mods) => {
                        keys.insert(
                            key_sequence,
                            (
                                key_mods,
                                Message::Move(direction, amount * kdl_config.movement_multiplier),
                            ),
                        );
                    }
                    Key::Extend(direction, amount, key_sequence, key_mods) => {
                        keys.insert(
                            key_sequence,
                            (
                                key_mods,
                                Message::Extend(direction, amount * kdl_config.movement_multiplier),
                            ),
                        );
                    }
                    Key::Shrink(direction, amount, key_sequence, key_mods) => {
                        keys.insert(
                            key_sequence,
                            (
                                key_mods,
                                Message::Shrink(direction, amount * kdl_config.movement_multiplier),
                            ),
                        );
                    }
                }
            }

            Config {
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

/// Utility macro to create a theme with default colors
///
/// Implementing the `Default` trait would be a lot of repetition and it cannot be automatically
/// derived. We want `Default` trait to be the same as what we specify for knus (KDL values defaults
/// if not specified)
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
            pub keys: Keys,
        }

        impl Default for KdlConfig {
            fn default() -> Self {
                Self {
                    theme: Theme::default(),
                    keys: Keys::default(),
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
            pub keys: std::collections::HashMap<KeySequence, (KeyMods, crate::message::Message)>,
        }

        impl Default for Config {
            fn default() -> Self {
                Self {
                    theme: Theme::default(),
                    keys: std::collections::HashMap::default(),
                    $(
                        $key: $default
                    ),*
                }
            }
        }
    }
}

config! {
    /// Specifying this option will copy the selection to clipboard as soon as you select your first rectangle.
    /// This is useful, since often times you may not want to make any modifications to your selection,
    /// so this makes simple select and copy faster.
    ///
    /// When this is `true`, while you are selecting the first square pressing the Right mouse button just once will
    /// cancel this effect and not instantly copy the screenshot.
    instant: bool = false,
    /// The default image service to use when uploading images to the internet.
    /// We have multiple options because some of them can be down / unreliable etc.
    ///
    /// You may also get rate limited by the service if you send too many images, so you can try a different
    /// one if that happens.
    default_image_upload_provider: ImageUploadService = ImageUploadService::TheNullPointer,
    /// Renders a size indicator in the bottom left corner.
    /// It shows the current height and width of the selection.
    ///
    /// You can manually enter a value to change the selection by hand.
    size_indicator: bool = true,
    /// Say you have this keybinding
    ///
    /// ```kdl
    /// keys {
    ///   move "up" 5 "w"
    /// }
    /// ```
    ///
    /// The amount of pixels this actually moves by when pressing `w`
    /// depends on `movement_multiplier`.
    /// - `1`: 5px moved
    /// - `10`: 50px moved
    /// - `22`: 110px moved
    ///
    /// This applies to all keybindings that take a number like this.
    movement_multiplier: u32 = 120
}

theme! {
    /// Color of text which is placed in contrast with the color of `accent_bg`
    accent_fg = iced::color!(0x_ab_61_37),
    /// The background color of icons, the selection and such
    accent = iced::Color::WHITE,
}

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
