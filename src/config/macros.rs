//! Macros used to declare the configuration keys

/// Represents the color node used in the KDL config, to be parsed into
/// this structure.
///
/// # Examples
///
/// ```kdl
/// theme {
///   // an opaque white color
///   background ffffff
///   // black color with 50% opacity
///   foreground 000000 opacity=0.5
/// }
/// ```
#[derive(ferrishot_knus::Decode, Debug)]
pub struct Color {
    /// Hex color. Examples:
    ///
    /// - `ff0000`: Red
    /// - `000000`: Black
    #[ferrishot_knus(argument)]
    pub color: u32,
    /// The opacity for this color.
    /// - `1.0`: Opaque
    /// - `0.0`: Transparent
    #[ferrishot_knus(default = 1.0, property)]
    pub opacity: f32,
}

/// A place on the rectangle
#[derive(ferrishot_knus::DecodeScalar, Debug, Clone)]
pub enum Place {
    /// Center
    Center,
    /// Center on the x-axis
    XCenter,
    /// Center on the y-axis
    YCenter,
    /// Top-left corner
    TopLeft,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-right corner
    BottomRight,
    /// Top-right corner
    TopRight,
    /// Left side
    Left,
    /// Right side
    Right,
    /// Top side
    Top,
    /// Bottom side
    Bottom,
}

/// Declare config options
///
/// `UserKdlConfig` is merged into `DefaultKdlConfig` before being processed
/// into a `Config`
#[macro_export]
macro_rules! declare_config_options {
    (
        $(
            $(#[$doc:meta])*
            $key:ident: $typ:ty
        ),* $(,)?
    ) => {
        /// Configuration for ferrishot.
        #[derive(Debug)]
        pub struct Config {
            /// Ferrishot's theme and colors
            pub theme: Theme,
            /// Ferrishot's keybindings
            pub keys: $crate::config::key::KeyMap,
            $(
                $(#[$doc])*
                pub $key: $typ,
            )*
        }

        /// The default config as read from the default config file, included as a static string in the binary.
        /// All values are required and must be specified
        #[derive(ferrishot_knus::Decode, Debug)]
        pub struct DefaultKdlConfig {
            /// The default keybindings of ferrishot
            #[ferrishot_knus(child)]
            pub keys: $crate::config::key::Keys,
            /// The default theme of ferrishot
            #[ferrishot_knus(child)]
            pub theme: DefaultKdlTheme,
            $(
                $(#[$doc])*
                #[ferrishot_knus(child, unwrap(argument))]
                pub $key: $typ,
            )*
        }

        impl DefaultKdlConfig {
            /// Merge the user's top-level config options with the default options.
            /// User config options take priority.
            pub fn merge_user_config(mut self, user_config: UserKdlConfig) -> Self {
                $(
                    self.$key = user_config.$key.unwrap_or(self.$key);
                )*
                // merge keybindings
                //
                // If the same keybinding is defined in the default theme and
                // the user theme, e.g.
                //
                // default:
                //
                // ```kdl
                // keys {
                //   goto top-left key=gg
                // }
                // ```
                //
                // user:
                //
                // ```kdl
                // keys {
                //   goto bottom-right key=gg
                // }
                // ```
                //
                // The user's keybinding will come after. Since we are iterating over the
                // keys sequentially, and inserting into the `KeyMap` one-by-one, the default keybinding
                // will be inserted into the `KeyMap`, but it will be overridden by the user keybinding.
                //
                // Essentially what we want to make sure is that if the same key is defined twice,
                // the user keybinding takes priority.
                self
                    .keys
                    .keys
                    .extend(user_config.keys.unwrap_or_default().keys);

                if let Some(user_theme) = user_config.theme {
                    self.theme = self.theme.merge_user_theme(user_theme);
                };

                self
            }
        }

        impl From<DefaultKdlConfig> for Config {
            fn from(value: DefaultKdlConfig) -> Self {
                Self {
                    $(
                        $key: value.$key,
                    )*
                    theme: value.theme.into(),
                    keys: value.keys.keys.into_iter().collect::<$crate::config::KeyMap>(),
                }
            }
        }

        /// User's config. Everything is optional. Values will be merged with `DefaultKdlConfig`.
        /// And will take priority over the default values.
        #[derive(ferrishot_knus::Decode, Debug)]
        pub struct UserKdlConfig {
            /// User-defined keybindings
            #[ferrishot_knus(child)]
            pub keys: Option<$crate::config::key::Keys>,
            /// User-defined colors
            #[ferrishot_knus(child)]
            pub theme: Option<UserKdlTheme>,
            $(
                $(#[$doc])*
                #[ferrishot_knus(child, unwrap(argument))]
                pub $key: Option<$typ>,
            )*
        }
    }
}

/// Declare theme keys
///
/// `UserKdlTheme` is merged into `DefaultKdlTheme` before being processed
/// into a `Theme`
#[macro_export]
macro_rules! declare_theme_options {
    (
        $(
            $(#[$doc:meta])*
            $key:ident
        ),* $(,)?
    ) => {
        /// Theme and colors of ferrishot
        #[derive(Debug, Copy, Clone)]
        pub struct Theme {
            $(
                $(#[$doc])*
                pub $key: iced::Color,
            )*
        }

        /// Ferrishot's default theme and colors
        #[derive(ferrishot_knus::Decode, Debug)]
        pub struct DefaultKdlTheme {
            $(
                $(#[$doc])*
                #[ferrishot_knus(child)]
                pub $key: $crate::config::Color,
            )*
        }

        impl DefaultKdlTheme {
            /// If the user theme specifies a color, it will override the color in the
            /// default theme.
            pub fn merge_user_theme(mut self, user_theme: UserKdlTheme) -> Self {
                $(
                    self.$key = user_theme.$key.unwrap_or(self.$key);
                )*
                self
            }
        }

        impl From<DefaultKdlTheme> for Theme {
            fn from(value: DefaultKdlTheme) -> Self {
                Self {
                    $(
                        $key: {
                            let [.., r, g, b] = value.$key.color.to_be_bytes();
                            iced::Color::from_rgba(
                                f32::from(r) / 255.0,
                                f32::from(g) / 255.0,
                                f32::from(b) / 255.0,
                                value.$key.opacity
                            )
                        },
                    )*
                }
            }
        }

        /// The user's custom theme and color overrides
        /// All values are optional and will override whatever is the default
        #[derive(ferrishot_knus::Decode, Debug)]
        pub struct UserKdlTheme {
            $(
                $(#[$doc])*
                #[ferrishot_knus(child)]
                pub $key: Option<$crate::config::Color>,
            )*
        }
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
macro_rules! declare_key_options {
    (
        $(
            $(#[$Keymappable_Command_Attr:meta])*
            $Keymappable_Command:ident $({$(
                $(#[$Command_Argument_Attr:meta])*
                $Command_Argument:ident: $Command_Argument_Ty:ty $(= $Command_Argument_Default:expr)?,
            )+})?
        ),* $(,)?
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

        impl KeymappableCommand {
            /// Obtain the Action for this key. What will happen when the specific `KeySequence` is fired
            /// provided that the `KeyMods` match the current key modifiers.
            pub fn action(self) -> (($crate::config::key::KeySequence, $crate::config::key::KeyMods), Command) {
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

        /// An action in the app
        #[derive(Debug, Clone)]
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
    }
}
