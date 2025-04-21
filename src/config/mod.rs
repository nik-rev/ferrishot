//! Configuration of ferrishot
//!
//! Uses KDL as the configuration language <https://kdl.dev/>
//!
//! The user's config (`UserKdlConfig`) is merged into the default Kdl configuration
//! (`DefaultKdlConfig`). This is then transformed into a `Config` by doing a little bit of
//! extra processing for things that could not be trivially determined during deserialization.
//! Such as:
//! - Converting the list of keybindings into a structured `KeyMap` which can be indexed `O(1)` to
//!   obtain the `Message` to execute for that action.
//! - Adding opacity to colors
//!
//! ---
//!
//! When modifying the config options, make sure to update the `default-config.kdl` file

pub mod cli;
pub mod key;
pub mod macros;
pub mod named_key;

/// The default configuration for ferrishot, to be merged with the user's config
pub const DEFAULT_KDL_CONFIG_STR: &str = include_str!("../../default.kdl");

use crate::config::key::KeyMap;
use crate::config::macros::Color;
use crate::corners::Direction;
use crate::image_upload::ImageUploadService;
pub use cli::CLI;
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;

crate::declare_config_options! {
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

crate::declare_key_options! {
    /// Copy the selected region as a screenshot to the clipboard
    CopyToClipboard,
    /// Save the screenshot as a path
    SaveScreenshot,
    /// Exit the application
    Exit,
    /// Set selection to encompass the entire screen
    SelectFullScreen,
    /// Shift the selection in the given direction by pixels
    Move {
        direction: Direction,
        amount: u32,
    },
    /// Increase the size of the selection in the given direction by pixels
    Extend {
        direction: Direction,
        amount: u32,
    },
    /// Decrease the size of the selection in the given direction by pixels
    Shrink {
        direction: Direction,
        amount: u32,
    },
}

crate::declare_theme_options! {
    /// Color of the border around the selection
    selection_frame,
    /// Color of the region outside of the selected area
    non_selected_region,
    /// Color of drop shadow, used for stuff like:
    ///
    /// - drop shadow of icons
    /// - drop shadow of selection rectangle
    /// - drop shadow around error box
    drop_shadow,
    /// Background color of selected text
    text_selection,
    /// Foreground color of the size indicator
    size_indicator_fg,
    /// Background color of the size indicator
    size_indicator_bg,
    /// Text color of the tooltip
    tooltip_fg,
    /// Background color of the tooltip
    tooltip_bg,
    /// Color of the text on errors
    error_fg,
    /// Background color of the error boxes
    error_bg,
    /// Background color of the info box, which shows various tips
    info_box_bg,
    /// Text color of the info box, which shows various tips
    info_box_fg,
    /// Background color of the icons around the selection
    icon_bg,
    /// Color of icons around the selection
    icon_fg,
}

/// Configuration of the app
///
/// Static as it will never change once the app is launched.
/// It also makes it easy to get the config values anywhere from the app, even where we don't have access to
/// the `App`.
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let kdl_config = (|| -> miette::Result<DefaultKdlConfig> {
        let config_file = CLI.config_file.as_str();
        let config_file_path = PathBuf::from(config_file);

        let default_config =
            knus::parse::<DefaultKdlConfig>("<default-config>", DEFAULT_KDL_CONFIG_STR)?;

        // if there is no config file, act as if it's simply empty
        let user_config = knus::parse::<UserKdlConfig>(
            &CLI.config_file,
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
