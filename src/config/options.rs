//! Declare config options

use crate::config::Place;
use crate::geometry::Direction;
use crate::lazy_rect::LazyRectangle;

crate::declare_config_options! {
    /// Renders a size indicator in the bottom left corner.
    /// It shows the current height and width of the selection.
    ///
    /// You can manually enter a value to change the selection by hand.
    size_indicator: bool,
    /// Render icons around the selection
    selection_icons: bool,
}

crate::declare_key_options! {
    /// Do nothing
    NoOp,
    /// Open the keybindings cheatsheet
    OpenKeybindingsCheatsheet,
    /// Toggle the overlay showing various information for debugging
    ToggleDebugOverlay,
    /// Open a grid of letters to pick the top left corner in 3 keystrokes
    PickTopLeftCorner,
    /// Open a grid of letters to pick the bottom right corner in 3 keystrokes
    PickBottomRightCorner,
    /// Upload screenshot to the internet
    UploadScreenshot,
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
    SelectRegion {
        #[ferrishot_knus(str)]
        selection: LazyRectangle,
    },
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
    /// Cheatsheet background
    cheatsheet_bg,
    /// Cheatsheet text color
    cheatsheet_fg,

    /// Close the popup
    popup_close_icon_bg,
    /// Cheatsheet text color
    popup_close_icon_fg,

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

    //
    // --- Side Indicator ---
    //
    /// Foreground color of the size indicator
    size_indicator_fg,
    /// Background color of the size indicator
    size_indicator_bg,

    //
    // --- Tooltip ---
    //
    /// Text color of the tooltip
    tooltip_fg,
    /// Background color of the tooltip
    tooltip_bg,

    //
    // --- Errors ---
    //
    /// Color of the text on errors
    error_fg,
    /// Background color of the error boxes
    error_bg,

    //
    // --- Info Box ---
    //
    /// Background color of the info box, which shows various tips
    info_box_bg,
    /// Text color of the info box, which shows various tips
    info_box_fg,
    /// Color of the border of the info box
    info_box_border,

    //
    // --- Selection Icons ---
    //
    /// Background color of the icons around the selection
    icon_bg,
    /// Color of icons around the selection
    icon_fg,

    //
    // --- Debug Menu ---
    //
    /// Color of the labels in the debug menu (F12)
    debug_label,
    /// Foreground color of debug menu (F12)
    debug_fg,
    /// Background color of debug menu (F12)
    debug_bg,

    //
    // --- Letters ---
    //
    /// Color of lines
    letters_lines,
    /// Color of letters
    letters_fg,
    /// Background color of letters
    letters_bg,

    //
    // --- Image uploaded popup ---
    //
    /// Foreground color of the image_uploaded popup
    image_uploaded_fg,
    /// Background color of the image_uploaded popup
    image_uploaded_bg,

    /// Color of success, e.g. green check mark when copying text to clipboard
    success,
}
