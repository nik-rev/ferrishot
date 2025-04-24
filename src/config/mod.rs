//! Configuration of ferrishot
//!
//! Uses KDL as the configuration language <https://kdl.dev/>
//!
//! The user's config (`UserKdlConfig`) is merged into the default Kdl configuration
//! (`DefaultKdlConfig`). Both of these structs and more are created in this file using
//! macros found in `macros.rs`. The macros are necessary to avoid a lot of boilerplate.
//!
//! The `DefaultKdlConfig` is then transformed into a `Config` by doing a little bit of
//! extra processing for things that could not be trivially determined during deserialization.
//!
//! Such as:
//! - Converting the list of keybindings into a structured `KeyMap` which can be indexed `O(1)` to
//!   obtain the `Message` to execute for that action.
//! - Adding opacity to colors

mod cli;
mod key;
mod macros;
mod named_key;
mod options;

use crate::config::key::KeyMap;
use crate::config::macros::Color;

use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;

use options::{DefaultKdlConfig, UserKdlConfig};

pub use cli::{CLI, DEFAULT_LOG_FILE_PATH};
pub use macros::Place;
pub use options::{Config, Key, KeyAction};

/// The default configuration for ferrishot, to be merged with the user's config
///
/// When modifying any of the config options, this will also need to be updated
pub const DEFAULT_KDL_CONFIG_STR: &str = include_str!("../../default.kdl");

/// Configuration of the app
///
/// Static as it will never change once the app is launched.
/// It also makes it easy to get the config values anywhere from the app, even where we don't have access to the `App`.
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let kdl_config = (|| -> miette::Result<DefaultKdlConfig> {
        let config_file = CLI.config_file.as_str();
        let config_file_path = PathBuf::from(config_file);

        let default_config =
            knus::parse::<DefaultKdlConfig>("<default-config>", DEFAULT_KDL_CONFIG_STR)?;

        let user_config = knus::parse::<UserKdlConfig>(
            &CLI.config_file,
            // if there is no config file, act as if it's simply empty
            &fs::read_to_string(&config_file_path).unwrap_or_default(),
        )?;

        Ok(default_config.merge_user_config(user_config))
    })();

    match kdl_config {
        Ok(kdl_config) => Config {
            instant: kdl_config.instant,
            default_image_upload_provider: kdl_config.default_image_upload_provider,
            size_indicator: kdl_config.size_indicator,
            theme: kdl_config.theme.into(),
            keys: kdl_config.keys.keys.into_iter().collect::<KeyMap>(),
        },
        Err(miette_error) => {
            eprintln!("{miette_error:?}");
            std::process::exit(1);
        }
    }
});
