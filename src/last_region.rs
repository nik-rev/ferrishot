//! Read and write the last region of a rectangle
use crate::{
    geometry::RectangleExt as _,
    lazy_rect::{LazyRectangle, ParseRectError},
};
use etcetera::BaseStrategy as _;
use iced::Rectangle;
use std::{fs, io::Write as _, str::FromStr as _};
use tap::Pipe as _;

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
/// Name of the file used to read the last region
pub const LAST_REGION_FILENAME: &str = "ferrishot-last-region.txt";

/// Read the last region used
pub fn read(image_bounds: Rectangle) -> Result<Option<Rectangle>, Error> {
    etcetera::choose_base_strategy()?
        .cache_dir()
        .join(LAST_REGION_FILENAME)
        .pipe(fs::read_to_string)?
        .pipe_deref(LazyRectangle::from_str)?
        .pipe(|lazy_rect| lazy_rect.init(image_bounds))
        .pipe(Some)
        .pipe(Ok)
}

/// Write the last used region
pub(crate) fn write(region: Rectangle) -> Result<(), Error> {
    etcetera::choose_base_strategy()?
        .cache_dir()
        .join(LAST_REGION_FILENAME)
        .pipe(fs::File::create)?
        .write_all(region.as_str().as_bytes())?
        .pipe(Ok)
}
