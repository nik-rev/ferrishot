//! Declare config options

use crate::config::Place;
use crate::image_upload::ImageUploadService;
use crate::rect::Direction;

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
    /// Toggle the overlay showing various information for debugging
    ToggleDebugOverlay,
    /// Open a grid of letters to pick the top left corner in 3 keystrokes
    PickTopLeftCorner,
    /// Open a grid of letters to pick the bottom right corner in 3 keystrokes
    PickBottomRightCorner,
    /// Copy the selected region as a screenshot to the clipboard
    CopyToClipboard,
    /// Save the screenshot as a path
    SaveScreenshot,
    /// Set the width to whatever number is currently pressed
    SetWidth,
    /// Set the height to whatever number is currently pressed
    SetHeight,
    /// Exit the application
    Exit,
    /// Set selection to encompass the entire screen
    SelectFullScreen,
    /// Remove the selection
    ClearSelection,
    /// Shift the selection in the given direction by pixels
    Move {
        direction: Direction,
        amount: u32 = u32::MAX,
    },
    /// Increase the size of the selection in the given direction by pixels
    Extend {
        direction: Direction,
        amount: u32 = u32::MAX,
    },
    /// Decrease the size of the selection in the given direction by pixels
    Shrink {
        direction: Direction,
        amount: u32 = u32::MAX,
    },
    /// Move rectangle to a place
    Goto {
        place: Place,
    }
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
