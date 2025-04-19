//! Configuration of ferrishot
use std::{io::Read as _, sync::LazyLock};

use clap::Parser;
use miette::IntoDiagnostic as _;

use crate::{
    corners::{Direction, RectPlace},
    image_upload::ImageUploadService,
    key::KeySequence,
    theme::Color,
};

/// Configuration of the app
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let config = (|| -> miette::Result<Config> {
        let mut buf = String::new();

        std::io::stdin()
            .read_to_string(&mut buf)
            .into_diagnostic()?;

        Ok(knus::parse::<Config>("<stdin>", &buf)?)
    })();

    match config {
        Ok(config) => config,
        Err(miette_error) => {
            eprintln!("{miette_error:?}");
            std::process::exit(1);
        }
    }
});

/// Config
#[derive(knus::Decode, Debug)]
pub struct Config {
    /// Settings
    #[knus(child)]
    pub settings: Settings,
    /// Theme
    #[knus(child)]
    pub theme: Theme,
    /// Keybindings
    #[knus(child)]
    pub keys: Keys,
}

/// Settings
#[derive(knus::Decode, Debug)]
pub struct Settings {
    /// Specifying this option will copy the selection to clipboard as soon as you select your first rectangle.
    /// This is useful, since often times you may not want to make any modifications to your selection,
    /// so this makes simple select and copy faster.
    ///
    /// When this is `true`, while you are selecting the first square pressing the Right mouse button just once will
    /// cancel this effect and not instantly copy the screenshot.
    #[knus(default = false, child, unwrap(argument))]
    pub instant: bool,
    /// The default image service to use when uploading images to the internet.
    /// We have multiple options because some of them can be down / unreliable etc.
    ///
    /// You may also get rate limited by the service if you send too many images, so you can try a different
    /// one if that happens.
    #[knus(default = ImageUploadService::TheNullPointer, child, unwrap(argument))]
    pub default_image_upload_provider: ImageUploadService,
    /// Renders a size indicator in the bottom left corner.
    /// It shows the current height and width of the selection.
    ///
    /// You can manually enter a value to change the selection by hand.
    #[knus(default = true, child, unwrap(argument))]
    pub size_indicator: bool,
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
    #[knus(default = 120, child, unwrap(argument))]
    pub movement_multiplier: u32,
}

/// Theme
#[derive(knus::Decode, Debug)]
pub struct Theme {
    /// Color of text which is placed in contrast with the color of `accent_bg`
    #[knus(default = Color(iced::color!(0x_ab_61_37)), child, unwrap(argument, str))]
    pub accent_fg: Color,
    /// The background color of icons, the selection and such
    #[knus(default = Color(iced::Color::WHITE), child, unwrap(argument, str))]
    pub accent: Color,
}

/// Keybindings for ferrishot
#[derive(knus::Decode, Debug)]
pub struct Keys {
    /// A list of keybindings for ferrishot
    #[knus(children)]
    pub keys: Vec<Key>,
}

/// A list of keybindings which exist in the app
#[derive(knus::Decode, Debug)]
pub enum Key {
    /// Copy the selected region as a screenshot to the clipboard
    CopyToClipboard(#[knus(argument, str)] KeySequence),
    /// Save the screenshot as a path
    SaveScreenshot(#[knus(argument, str)] KeySequence),
    /// Exit the application
    Exit(#[knus(argument, str)] KeySequence),
    /// Teleport the selection to the given area
    Goto(
        // where to move the rect
        #[knus(argument, str)] RectPlace,
        // binding
        #[knus(argument, str)] KeySequence,
    ),
    /// Shift the selection in the given direction by pixels
    Move(
        #[knus(argument)] Direction,
        // strength
        #[knus(argument)] u32,
        // binding
        #[knus(argument, str)] KeySequence,
    ),
    /// Increase the size of the selection in the given direction by pixels
    Extend(
        // where to extend
        #[knus(argument)] Direction,
        // strength
        #[knus(argument)] u32,
        // binding
        #[knus(argument, str)] KeySequence,
    ),
    /// Decrease the size of the selection in the given direction by pixels
    Shrink(
        // shrink in this direction
        #[knus(argument)] Direction,
        // strength
        #[knus(argument)] u32,
        // binding
        #[knus(argument, str)] KeySequence,
    ),
}

/// Configuration for the program
#[derive(Parser, Debug)]
#[command(version, about, author = "Nik Revenco")]
pub struct Cli {
    /// The first selection will be copied to the clipboard as soon as the left mouse button is released
    #[arg(long)]
    pub instant: bool,
}

// Pencil(#[knus(argument, str)] KeySequence),
// Line(#[knus(argument, str)] KeySequence),
// Square(#[knus(argument, str)] KeySequence),
// Rectangle(#[knus(argument, str)] KeySequence),
// Circle(#[knus(argument, str)] KeySequence),
// Marker(#[knus(argument, str)] KeySequence),
// Text(#[knus(argument, str)] KeySequence),
// Undo(#[knus(argument, str)] KeySequence),
// Redo(#[knus(argument, str)] KeySequence),
