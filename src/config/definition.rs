use crate::config::DefaultKdlConfig;
use crate::config::UserKdlConfig;
use std::{path::PathBuf, sync::LazyLock};

use clap::Parser;
use etcetera::BaseStrategy;

/// Represents the location of the config file
///
/// This is only accurate until `Cli::parse` is invoked (see `ferrishot::CLI`).
/// After that, it can be incorrect if the user passes `--config-file` option
static CONFIG_FILE_BEFORE_CLI: LazyLock<PathBuf> = LazyLock::new(|| {
    etcetera::choose_base_strategy().map_or_else(
        |_| PathBuf::from("ferrishot.kdl"),
        |strategy| strategy.config_dir().join("ferrishot").join("config.kdl"),
    )
});

/// Command line arguments for the program
#[derive(Parser, Debug)]
#[command(version, about, author = "Nik Revenco")]
pub struct Cli {
    /// Write the default config file
    #[arg(long, help = format!("Write the default config to {}", CONFIG_FILE_BEFORE_CLI.display()))]
    pub dump_default_config: bool,
    /// Specifies the config file to use
    #[arg(
        long,
        value_name = "file.kdl",
        default_value_t = CONFIG_FILE_BEFORE_CLI.to_string_lossy().to_string()
    )]
    pub config_file: String,
}

/// Command line arguments to this program
///
/// It is a static because it is needed by the `CONFIG` static, in order to
/// read config from the correct place
pub static CLI: LazyLock<Cli> = LazyLock::new(Cli::parse);

/// The default configuration for ferrishot, to be *merged* with the user's config
pub const DEFAULT_KDL_CONFIG_STR: &str = include_str!("../../default-config.kdl");

impl DefaultKdlConfig {
    /// Merge the user's config with the default config
    pub fn merge_user_config(mut self, user_config: UserKdlConfig) -> Self {
        if let Some(keys) = user_config.keys {
            self.keys.keys.extend(keys.keys);
        }

        if let Some(theme) = user_config.theme {
            if let Some(accent_fg) = theme.accent_fg {
                self.theme.accent_fg = accent_fg;
            }

            if let Some(accent) = theme.accent {
                self.theme.accent = accent;
            }
        }

        if let Some(instant) = user_config.instant {
            self.instant = instant;
        }

        if let Some(default_image_upload_provider) = user_config.default_image_upload_provider {
            self.default_image_upload_provider = default_image_upload_provider;
        }

        if let Some(size_indicator) = user_config.size_indicator {
            self.size_indicator = size_indicator;
        }

        self
    }
}

/// Represents the color in the KDL config
#[derive(knus::Decode, Debug)]
pub struct Color {
    /// The underlying color
    #[knus(argument, str)]
    pub color: crate::theme::Color,
    /// The opacity for this color.
    /// - `1.0`: Opaque
    /// - `0.0`: Transparent
    #[knus(default, property)]
    pub opacity: f32,
}

/// Declare config options
#[macro_export]
macro_rules! declare_config_options {
    (
        $(
            $(#[$doc:meta])*
            $key:ident: $typ:ty
        ),* $(,)?
    ) => {
        /// The default config as read from the default config file, included as a static string in the binary.
        /// All values are required and must be specified
        #[derive(knus::Decode, Debug)]
        pub struct DefaultKdlConfig {
            /// The default keybindings of ferrishot
            #[knus(child)]
            pub keys: $crate::config::key::Keys,
            /// The default theme of ferrishot
            #[knus(child)]
            pub theme: DefaultKdlTheme,
            $(
                $(#[$doc])*
                #[knus(child, unwrap(argument))]
                pub $key: $typ,
            )*
        }
        /// User's config. Everything is optional. Values will be merged with `DefaultKdlConfig`.
        /// And will take priority over the default values.
        #[derive(knus::Decode, Debug)]
        pub struct UserKdlConfig {
            /// User-defined keybindings
            #[knus(child)]
            pub keys: Option<$crate::config::key::Keys>,
            /// User-defined colors
            #[knus(child)]
            pub theme: Option<UserKdlTheme>,
            $(
                $(#[$doc])*
                #[knus(child, unwrap(argument))]
                pub $key: Option<$typ>,
            )*
        }
        /// Configuration for ferrishot.
        #[derive(Debug)]
        pub struct Config {
            /// Ferrishot's theme and colors
            pub theme: Theme,
            /// Ferrishot's keybindings
            pub keys: $crate::config::key::KeyMap,
            $(
                $(#[$doc])*
                pub $key: $typ,
            )*
        }
    }
}

/// Declare theme keys
#[macro_export]
macro_rules! declare_theme_options {
    (
        $(
            $(#[$doc:meta])*
            $key:ident
        ),* $(,)?
    ) => {
        /// Ferrishot's default theme and colors
        #[derive(knus::Decode, Debug)]
        pub struct DefaultKdlTheme {
            $(
                $(#[$doc])*
                #[knus(child)]
                pub $key: $crate::config::Color,
            )*
        }

        impl From<DefaultKdlTheme> for Theme {
            fn from(value: DefaultKdlTheme) -> Self {
                Self {
                    $(
                        $key: value.$key.color.with_opacity(value.$key.opacity),
                    )*
                }
            }
        }

        /// The user's custom theme and color overrides
        /// All values are optional and will override whatever is the default
        #[derive(knus::Decode, Debug)]
        pub struct UserKdlTheme {
            $(
                $(#[$doc])*
                #[knus(child)]
                pub $key: Option<$crate::config::Color>,
            )*
        }

        /// Theme and colors of ferrishot
        #[derive(Debug)]
        pub struct Theme {
            $(
                $(#[$doc])*
                pub $key: $crate::theme::Color,
            )*
        }
    }
}

/// Create keybindings, specifying the arguments it receives as named fields.
/// Each keybind is declared like this:
///
/// ```text
/// Keybind {
///     a: u32
///     b: bool
///     c: f32
///     d: String
/// }
/// ```
///
/// The above creates a new keybind that will take 4 arguments in order, of the respective types.
/// It can be used in the `config.kdl` file like so:
///
/// ```kdl
/// keys {
///   keybind 10 #false 0.8 hello key=g mods=ctrl
/// }
/// ```
///
/// Which generates a structure like so, when parsed:
///
/// ```no_compile
/// Key::Keybind(10, false, 0.8, "hello", KeySequence("g", None), KeyMods::CTRL)
/// ```
#[macro_export]
macro_rules! declare_key_options {
    (
        $(
            $(#[$doc:meta])*
            $KeyOption:ident $({
                $(
                    $(#[$attr:meta])*
                    $field:ident: $Argument:ty,
                )+
            })?
        ),* $(,)?
    ) => {
        /// A list of keybindings which exist in the app
        ///
        /// These have just been parsed, they are
        #[derive(knus::Decode, Debug, Clone)]
        pub enum Key {
            $(
                $(#[$doc])*
                $KeyOption(
                    $($(
                        $(#[$attr])*
                        #[knus(argument)] $Argument,
                    )*)?
                    #[knus(property(name = "key"), str)] $crate::config::key::KeySequence,
                    #[knus(default, property(name = "mods"), str)] $crate::config::key::KeyMods,
                ),
            )*
        }

        impl Key {
            /// Obtain the Action for this key. What will happen when the specific `KeySequence` is fired
            /// provided that the `KeyMods` match the current key modifiers.
            pub fn action(self) -> ($crate::config::key::KeySequence, ($crate::config::key::KeyMods, KeyAction)) {
                match self {
                    $(
                        Self::$KeyOption($($($field,)*)? key_sequence, key_mods) => {
                            (key_sequence, (key_mods, KeyAction::$KeyOption$(($($field),*))?))
                        },
                    )*
                }
            }
        }

        /// The action associated with a key
        #[derive(Debug, Clone)]
        pub enum KeyAction {
            $(
                $(#[$doc])*
                $KeyOption$(($($Argument,)*))?,
            )*
        }
    }
}
