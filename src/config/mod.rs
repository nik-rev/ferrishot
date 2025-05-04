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

pub use cli::{AcceptOnSelect, Cli};

use std::fs;
use std::path::PathBuf;

use options::{DefaultKdlConfig, UserKdlConfig};

pub use cli::DEFAULT_LOG_FILE_PATH;
pub use macros::Place;
pub use options::{Config, Key, KeyAction, Theme};

/// The default configuration for ferrishot, to be merged with the user's config
///
/// When modifying any of the config options, this will also need to be updated
pub const DEFAULT_KDL_CONFIG_STR: &str = include_str!("../../default.kdl");

impl Config {
    /// # Errors
    ///
    /// Default config, or the user's config is invalid
    pub fn parse(user_config: &str) -> Result<Self, miette::Error> {
        let config_file_path = PathBuf::from(user_config);

        let default_config =
            ferrishot_knus::parse::<DefaultKdlConfig>("<default-config>", DEFAULT_KDL_CONFIG_STR)?;

        let user_config = ferrishot_knus::parse::<UserKdlConfig>(
            &user_config,
            // if there is no config file, act as if it's simply empty
            &fs::read_to_string(&config_file_path).unwrap_or_default(),
        )?;

        Ok(default_config.merge_user_config(user_config).into())
    }
}
