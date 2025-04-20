//! Configuration of ferrishot
use std::{fs, path::PathBuf, sync::LazyLock};

use clap::Parser;
use etcetera::BaseStrategy;
use miette::IntoDiagnostic as _;

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
pub const DEFAULT_KDL_CONFIG_STR: &str = include_str!("../default-config.kdl");

/// Configuration of the app
///
/// Static as that means it will never change once the app is launched.
/// It also makes it easy to get the config values anywhere from the app, even where we don't have access to
/// the `App`.
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let kdl_config = (|| -> miette::Result<DefaultKdlConfig> {
        let config_file = CLI.config_file.as_str();
        let config_file_path = PathBuf::from(config_file);

        let default_config =
            knus::parse::<DefaultKdlConfig>("<default-config>", DEFAULT_KDL_CONFIG_STR)
                .expect("Default config is valid");

        // if there is no config file, act as if it's simply empty
        let user_config = knus::parse::<UserKdlConfig>(
            &CLI.config_file,
            &fs::read_to_string(&config_file_path)
                .into_diagnostic()
                .unwrap_or_default(),
        )?;

        Ok(default_config.merge_user_config(user_config))
    })();

    match kdl_config {
        Ok(kdl_config) => Config {
            instant: kdl_config.instant,
            default_image_upload_provider: kdl_config.default_image_upload_provider,
            size_indicator: kdl_config.size_indicator,
            theme: kdl_config.theme,
            keys: kdl_config
                .keys
                .keys
                .into_iter()
                .collect::<crate::key::KeyMap>(),
        },
        Err(miette_error) => {
            eprintln!("{miette_error:?}");
            std::process::exit(1);
        }
    }
});

use crate::image_upload::ImageUploadService;

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

/// Declare config options
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
            pub keys: $crate::key::Keys,
            /// The default theme of ferrishot
            #[knus(child)]
            pub theme: Theme,
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
            pub keys: Option<$crate::key::Keys>,
            /// User-defined colors
            #[knus(child)]
            pub theme: Option<UserKdlTheme>,
            $(
                $(#[$doc])*
                #[knus(child, unwrap(argument))]
                pub $key: Option<$typ>,
            )*
        }
        /// Configuration for ferrishot. This means the parsed Kdl has been through an extra
        /// processing step. What this means is that
        #[derive(Debug)]
        pub struct Config {
            /// Ferrishot's theme and colors
            pub theme: Theme,
            /// Ferrishot's keybindings
            pub keys: $crate::key::KeyMap,
            $(
                $(#[$doc])*
                pub $key: $typ,
            )*
        }
    }
}

/// Declare theme keys
macro_rules! declare_theme_options {
    (
        $(
            $(#[$doc:meta])*
            $key:ident
        ),* $(,)?
    ) => {
        /// The user's custom theme and color overrides
        /// All values are optional and will override whatever is the default
        #[derive(knus::Decode, Debug)]
        pub struct UserKdlTheme {
            $(
                #[knus(child, unwrap(argument, str))]
                pub $key: Option<$crate::theme::Color>,
            )*
        }
        /// Ferrishot's theme and colors
        #[derive(knus::Decode, Debug)]
        pub struct Theme {
            $(
                #[knus(child, unwrap(argument, str))]
                pub $key: $crate::theme::Color,
            )*
        }
    }
}

declare_theme_options! {
    /// Color of text which is placed in contrast with the color of `accent_bg`
    accent_fg,
    /// The background color of icons, the selection and such
    accent,
}

declare_config_options! {
    /// Specifying this option will copy the selection to clipboard as soon as you select your first rectangle.
    /// This is useful, since often times you may not want to make any modifications to your selection,
    /// so this makes simple select and copy faster.
    ///
    /// When this is `true`, while you are selecting the first square pressing the Right mouse button just once will
    /// cancel this effect and not instantly copy the screenshot.
    instant: bool,
    /// The default image service to use when uploading images to the internet.
    /// We have multiple options because some of them can be down / unreliable etc.
    ///
    /// You may also get rate limited by the service if you send too many images, so you can try a different
    /// one if that happens.
    default_image_upload_provider: ImageUploadService,
    /// Renders a size indicator in the bottom left corner.
    /// It shows the current height and width of the selection.
    ///
    /// You can manually enter a value to change the selection by hand.
    size_indicator: bool,
}
