//! Read and write the last region of a rectangle
use crate::geometry::{ParseRectError, RectangleExt as _};
use etcetera::BaseStrategy as _;
use iced::Rectangle;
use std::io::Write as _;
use tap::Pipe as _;

/// Read and write the last region of a rectangle
pub struct LastRegion;

/// Could not get the last region
#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum Error {
    /// Can't find home dir
    #[error(transparent)]
    HomeDir(#[from] etcetera::HomeDirError),
    /// Failed to parse as rectangle
    #[error(transparent)]
    ParseRect(#[from] ParseRectError),
    /// Failed to read the last region file
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl LastRegion {
    /// Name of the file used to read the last region
    pub const LAST_REGION_FILENAME: &str = "ferrishot-last-region.txt";

    /// Read the last region used
    pub fn read() -> Result<Option<Rectangle>, Error> {
        etcetera::choose_base_strategy()?
            .cache_dir()
            .join(Self::LAST_REGION_FILENAME)
            .pipe(std::fs::read_to_string)?
            .pipe_deref(Rectangle::from_str)?
            .pipe(Some)
            .pipe(Ok)
    }

    /// Write the last used region
    pub fn write(region: Rectangle) -> Result<(), Error> {
        etcetera::choose_base_strategy()?
            .cache_dir()
            .join(Self::LAST_REGION_FILENAME)
            .pipe(std::fs::File::create)?
            .write_all(region.as_str().as_bytes())?
            .pipe(Ok)
    }
}
