//! Configuration of ferrishot
use std::{io::Read, str::FromStr, sync::LazyLock};

use clap::Parser;
use miette::IntoDiagnostic;

use crate::{
    corners::{Direction, RectPlace},
    image_upload::ImageUploadService,
    theme::Color,
};

/// Configuration of the app
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let mut buf = String::new();

    std::io::stdin()
        .read_to_string(&mut buf)
        .into_diagnostic()
        .unwrap();

    knus::parse::<Config>("<stdin>", &buf).unwrap()
});

#[derive(knus::Decode, Debug)]
pub struct Config {
    #[knus(child)]
    pub settings: Settings,
    #[knus(child)]
    pub theme: Theme,
    #[knus(child)]
    pub keys: Keys,
}

#[derive(knus::Decode, Debug)]
pub struct Settings {
    #[knus(child, unwrap(argument))]
    pub instant: bool,
    #[knus(child, unwrap(argument))]
    pub default_image_upload_provider: ImageUploadService,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            instant: false,
            default_image_upload_provider: ImageUploadService::TheNullPointer,
        }
    }
}

#[derive(knus::Decode, Debug)]
pub struct Theme {
    #[knus(child, unwrap(argument, str))]
    pub accent_fg: Color,
    #[knus(child, unwrap(argument, str))]
    pub accent_bg: Color,
}

#[derive(knus::Decode, Debug)]
pub struct Keys {
    #[knus(children)]
    pub keys: Vec<Key>,
}

#[derive(Debug)]
pub struct KeySequence(String);

impl FromStr for KeySequence {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(String::from(s)))
    }
}

/// A list of keybindings which exist in the app
#[derive(knus::Decode, Debug)]
pub enum Key {
    /// Teleport the selection to the given area
    Goto(
        #[knus(argument, str)] RectPlace,
        #[knus(argument, str)] KeySequence,
    ),
    /// Shift the selection in the given direction by pixels
    Move(
        #[knus(argument)] Direction,
        // strength
        #[knus(argument)] u32,
        #[knus(argument, str)] KeySequence,
    ),
    /// Increase the size of the selection in the given direction by pixels
    Extend(
        #[knus(argument)] Direction,
        // strength
        #[knus(argument)] u32,
        #[knus(argument, str)] KeySequence,
    ),
    /// Decrease the size of the selection in the given direction by pixels
    Shrink(
        #[knus(argument)] Direction,
        // strength
        #[knus(argument)] u32,
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
