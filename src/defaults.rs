//! Default config for ferrishot

use crate::{
    corners::{Corner, Direction, RectPlace, Side, SideOrCorner},
    image_upload::ImageUploadService,
    key::KeyMap,
    message::Message,
};

crate::config! {
    /// Specifying this option will copy the selection to clipboard as soon as you select your first rectangle.
    /// This is useful, since often times you may not want to make any modifications to your selection,
    /// so this makes simple select and copy faster.
    ///
    /// When this is `true`, while you are selecting the first square pressing the Right mouse button just once will
    /// cancel this effect and not instantly copy the screenshot.
    instant: bool = false,
    /// The default image service to use when uploading images to the internet.
    /// We have multiple options because some of them can be down / unreliable etc.
    ///
    /// You may also get rate limited by the service if you send too many images, so you can try a different
    /// one if that happens.
    default_image_upload_provider: ImageUploadService = ImageUploadService::TheNullPointer,
    /// Renders a size indicator in the bottom left corner.
    /// It shows the current height and width of the selection.
    ///
    /// You can manually enter a value to change the selection by hand.
    size_indicator: bool = true,
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
    movement_multiplier: u32 = 120
}

crate::theme! {
    /// Color of text which is placed in contrast with the color of `accent_bg`
    accent_fg = iced::color!(0x_ab_61_37),
    /// The background color of icons, the selection and such
    accent = iced::Color::WHITE,
}

/// Default keymap
pub fn keymap() -> KeyMap {
    use Direction::{Down, Left, Right, Up};
    use Message::{Extend, Goto, Move, Shrink};
    crate::default_keys! {
        // direction
        "<" => Goto(RectPlace::SideOrCorner(SideOrCorner::Corner(Corner::BottomLeft))),
        ">" => Goto(RectPlace::SideOrCorner(SideOrCorner::Corner(Corner::TopRight))),
        "g c" => Goto(RectPlace::Center),
        "g g" => Goto(RectPlace::SideOrCorner(SideOrCorner::Corner(Corner::TopLeft))),
        "G" => Goto(RectPlace::SideOrCorner(SideOrCorner::Corner(Corner::BottomRight))),

        // up movements
        "g k" => Goto(RectPlace::SideOrCorner(SideOrCorner::Side(Side::Top))),

        "k" => Move(Up, 1),
        "K" => Extend(Up, 1),
        "k" mods="ctrl" => Shrink(Up, 1),

        "w" => Move(Up, 5),
        "W" => Extend(Up, 5),
        "w" mods="ctrl" => Shrink(Up, 5),

        // right movements
        "g l" => Goto(RectPlace::SideOrCorner(SideOrCorner::Side(Side::Right))),

        "l" => Move(Right, 1),
        "L" => Extend(Right, 1),
        "l" mods="ctrl" => Shrink(Right, 1),

        "e" => Move(Right, 5),
        "E" => Extend(Right, 5),
        "e" mods="ctrl" => Shrink(Right, 5),

        // left movements
        "g h" => Goto(RectPlace::SideOrCorner(SideOrCorner::Side(Side::Left))),

        "b" => Move(Left, 5),
        "B" => Extend(Left, 5),
        "b" mods="ctrl" => Shrink(Left, 5),

        "h" => Move(Left, 1),
        "H" => Extend(Left, 1),
        "h" mods="ctrl" => Shrink(Left, 1),

        // bottom movements
        "g j" => Goto(RectPlace::SideOrCorner(SideOrCorner::Side(Side::Bottom))),

        "j" => Move(Down, 1),
        "J" => Extend(Down, 1),
        "j" mods="ctrl" => Shrink(Down, 1),

        "n" => Move(Down, 5),
        "N" => Extend(Down, 5),
        "n" mods="ctrl" => Shrink(Down, 5),
    }
}
