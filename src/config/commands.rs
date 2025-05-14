//! Declare commands that can be invoked in the app

use crate::ui::popup::keybindings_cheatsheet;

use super::key::{KeyMods, KeySequence};

impl KeymappableCommand {
    /// Key sequence required for this command
    pub fn action(self) -> ((KeySequence, KeyMods), Command) {
        match self {
            Self::ImageUpload(cmd) => cmd.with_action(Command::ImageUpload),
            Self::App(cmd) => cmd.with_action(Command::App),
            Self::DebugOverlay(cmd) => cmd.with_action(Command::DebugOverlay),
            Self::KeybindingsCheatsheet(cmd) => cmd.with_action(Command::KeybindingsCheatsheet),
            Self::Letters(cmd) => cmd.with_action(Command::Letters),
            Self::Selection(cmd) => cmd.with_action(Command::Selection),
        }
    }
}

/// A list of keybindings which exist in the app
#[derive(Debug, Clone)]
pub enum KeymappableCommand {
    /// Image Upload
    ImageUpload(crate::image::action::KeymappableCommand),
    /// App
    App(crate::ui::app::KeymappableCommand),
    /// Debug overlay
    DebugOverlay(crate::ui::debug_overlay::KeymappableCommand),
    /// Keybindings Cheatsheet
    KeybindingsCheatsheet(keybindings_cheatsheet::KeymappableCommand),
    /// Letters
    Letters(crate::ui::popup::letters::KeymappableCommand),
    /// Selection
    Selection(crate::ui::selection::KeymappableCommand),
}

impl<S> ::ferrishot_knus::Decode<S> for KeymappableCommand
where
    S: ::ferrishot_knus::traits::ErrorSpan,
{
    fn decode_node(
        node: &::ferrishot_knus::ast::SpannedNode<S>,
        ctx: &mut ::ferrishot_knus::decode::Context<S>,
    ) -> ::std::result::Result<Self, ::ferrishot_knus::errors::DecodeError<S>> {
        // NOTE: it's unfortunate we have to repeat the node names here and the derive can't manually
        // take care of that.
        //
        // It is because we have 3-levels of indirection: OutmostEnum(OuterEnum(InnerEnum)).
        // `knus` only supports 2-levels of indirection
        //
        // TODO: Figure out how to automate this
        match &**node.node_name {
            // Screenshot
            "upload-screenshot" => ::ferrishot_knus::Decode::decode_node(node, ctx)
                .map(crate::image::action::KeymappableCommand::UploadScreenshot)
                .map(KeymappableCommand::ImageUpload),
            "copy-to-clipboard" => ::ferrishot_knus::Decode::decode_node(node, ctx)
                .map(crate::image::action::KeymappableCommand::CopyToClipboard)
                .map(KeymappableCommand::ImageUpload),
            "save-screenshot" => ::ferrishot_knus::Decode::decode_node(node, ctx)
                .map(crate::image::action::KeymappableCommand::SaveScreenshot)
                .map(KeymappableCommand::ImageUpload),

            // App
            "no-op" => ::ferrishot_knus::Decode::decode_node(node, ctx)
                .map(crate::ui::app::KeymappableCommand::NoOp)
                .map(KeymappableCommand::App),
            "exit" => ::ferrishot_knus::Decode::decode_node(node, ctx)
                .map(crate::ui::app::KeymappableCommand::Exit)
                .map(KeymappableCommand::App),

            // Debug overlay
            "toggle-debug-overlay" => ::ferrishot_knus::Decode::decode_node(node, ctx)
                .map(crate::ui::debug_overlay::KeymappableCommand::ToggleDebugOverlay)
                .map(KeymappableCommand::DebugOverlay),

            // Keybindings cheatsheet
            "open-keybindings-cheatsheet" => ::ferrishot_knus::Decode::decode_node(node, ctx)
                .map(keybindings_cheatsheet::KeymappableCommand::OpenKeybindingsCheatsheet)
                .map(KeymappableCommand::KeybindingsCheatsheet),

            // Letters
            "pick-top-left-corner" => ::ferrishot_knus::Decode::decode_node(node, ctx)
                .map(crate::ui::popup::letters::KeymappableCommand::PickTopLeftCorner)
                .map(KeymappableCommand::Letters),
            "pick-bottom-right-corner" => ::ferrishot_knus::Decode::decode_node(node, ctx)
                .map(crate::ui::popup::letters::KeymappableCommand::PickBottomRightCorner)
                .map(KeymappableCommand::Letters),

            // Selection
            "set-width" => ::ferrishot_knus::Decode::decode_node(node, ctx)
                .map(crate::ui::selection::KeymappableCommand::SetWidth)
                .map(KeymappableCommand::Selection),
            "set-height" => ::ferrishot_knus::Decode::decode_node(node, ctx)
                .map(crate::ui::selection::KeymappableCommand::SetHeight)
                .map(KeymappableCommand::Selection),
            "select-region" => ::ferrishot_knus::Decode::decode_node(node, ctx)
                .map(crate::ui::selection::KeymappableCommand::SelectRegion)
                .map(KeymappableCommand::Selection),
            "clear-selection" => ::ferrishot_knus::Decode::decode_node(node, ctx)
                .map(crate::ui::selection::KeymappableCommand::ClearSelection)
                .map(KeymappableCommand::Selection),
            "move" => ::ferrishot_knus::Decode::decode_node(node, ctx)
                .map(crate::ui::selection::KeymappableCommand::Move)
                .map(KeymappableCommand::Selection),
            "extend" => ::ferrishot_knus::Decode::decode_node(node, ctx)
                .map(crate::ui::selection::KeymappableCommand::Extend)
                .map(KeymappableCommand::Selection),
            "shrink" => ::ferrishot_knus::Decode::decode_node(node, ctx)
                .map(crate::ui::selection::KeymappableCommand::Shrink)
                .map(KeymappableCommand::Selection),
            "goto" => ::ferrishot_knus::Decode::decode_node(node, ctx)
                .map(crate::ui::selection::KeymappableCommand::Goto)
                .map(KeymappableCommand::Selection),

            _ => Err(::ferrishot_knus::errors::DecodeError::conversion(
                &node.node_name,
                "expected `move`, `extend`, or one of many others",
            )),
        }
    }
}

/// Command that can be triggered in the app
#[derive(Debug, Clone)]
pub enum Command {
    /// Image Upload
    ImageUpload(crate::image::action::Command),
    /// App
    App(crate::ui::app::Command),
    /// Debug Overlay
    DebugOverlay(crate::ui::debug_overlay::Command),
    /// Keybindings Cheatsheet
    KeybindingsCheatsheet(keybindings_cheatsheet::Command),
    /// Letters
    Letters(crate::ui::popup::letters::Command),
    /// Selection
    Selection(crate::ui::selection::Command),
}

impl crate::command::Handler for Command {
    fn handle(self, app: &mut crate::App, count: u32) -> iced::Task<crate::Message> {
        match self {
            Self::ImageUpload(command) => command.handle(app, count),
            Self::App(command) => command.handle(app, count),
            Self::DebugOverlay(command) => command.handle(app, count),
            Self::KeybindingsCheatsheet(command) => command.handle(app, count),
            Self::Letters(command) => command.handle(app, count),
            Self::Selection(command) => command.handle(app, count),
        }
    }
}

/// This command deserializes a key in the KDL file
pub trait KeymappableCommandTrait {
    /// The command that this evaluates to
    type Command: crate::command::Handler;

    /// Obtain the Action for this key. What will happen when the specific `KeySequence` is fired
    /// provided that the `KeyMods` match the current key modifiers.
    fn action(self) -> ((KeySequence, KeyMods), Self::Command);

    /// Transform the `action`
    fn with_action<Output, F: Fn(Self::Command) -> Output>(
        self,
        f: F,
    ) -> ((KeySequence, KeyMods), Output)
    where
        Self: Sized,
    {
        let (keys, cmd) = self.action();
        (keys, f(cmd))
    }
}

/// Create keybindings, specifying the arguments it receives as named fields.
/// Each keybind is declared like this:
///
/// ```text
/// Keybind {
///     a: u32
///     b: bool
///     c: f32
///     d: String
/// }
/// ```
///
/// The above creates a new keybind that will take 4 arguments in order, of the respective types.
/// It can be used in the `config.kdl` file like so:
///
/// ```kdl
/// keys {
///   keybind 10 #false 0.8 hello key=g mods=ctrl
/// }
/// ```
///
/// Which generates a structure like so, when parsed:
///
/// ```no_compile
/// Key::Keybind(10, false, 0.8, "hello", KeySequence("g", None), KeyMods::CTRL)
/// ```
#[macro_export]
macro_rules! declare_commands {
    (
        $(#[$Command_Attr:meta])*
        enum Command {
            $(
                $(#[$Keymappable_Command_Attr:meta])*
                $Keymappable_Command:ident $({$(
                    $(#[$Command_Argument_Attr:meta])*
                    $Command_Argument:ident: $Command_Argument_Ty:ty $(= $Command_Argument_Default:expr)?,
                )+})?
            ),* $(,)?
        }
    ) => {
        $(
            $(#[$Keymappable_Command_Attr])*
            #[derive(ferrishot_knus::Decode, Debug, Clone)]
            pub struct $Keymappable_Command {
                $($(
                    $(#[$Command_Argument_Attr])*
                    $(#[ferrishot_knus(default = $Command_Argument_Default)])?
                    #[ferrishot_knus(argument)]
                    $Command_Argument: $Command_Argument_Ty,
                )+)?
                #[ferrishot_knus(property(name = "key"), str)]
                keys: $crate::config::key::KeySequence,
                #[ferrishot_knus(default, property(name = "mod"), str)]
                mods: $crate::config::key::KeyMods,
            }
        )*

        /// A list of keybindings which exist in the app
        #[derive(ferrishot_knus::Decode, Debug, Clone)]
        pub enum KeymappableCommand {
            $(
                $Keymappable_Command($Keymappable_Command),
            )*
        }

        /// An action in the app
        #[allow(clippy::derive_partial_eq_without_eq, reason = "f32 cannot derive `Eq`")]
        #[derive(Debug, Clone, PartialEq, Copy)]
        $(#[$Command_Attr])*
        pub enum Command {
            $(
                $(#[$Keymappable_Command_Attr])*
                $Keymappable_Command $(
                    {
                        $(
                            $Command_Argument: $Command_Argument_Ty,
                        )*
                    }
                )?,
            )*
        }

        impl $crate::config::commands::KeymappableCommandTrait for KeymappableCommand {
            type Command = Command;

            fn action(self) -> (($crate::config::key::KeySequence, $crate::config::key::KeyMods), Self::Command) {
                match self {
                    $(
                        Self::$Keymappable_Command($Keymappable_Command {
                            $(
                                $($Command_Argument,)*
                            )?
                            keys,
                            mods
                        }) => {
                            (
                                (keys, mods),
                                Command::$Keymappable_Command$({
                                    $($Command_Argument),*
                                })?
                            )
                        },
                    )*
                }
            }
        }
    }
}
