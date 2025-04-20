//! Parse user keybindings

use crate::config::named_key::Named;
use std::{collections::HashMap, str::FromStr};

use iced::keyboard::{Modifiers, key::Key as IcedKey};

use crate::config::Key;

use super::KeyAction;

/// Represents the keybindings for ferrishot
///
/// # How to obtain the action for a specific key press
///
/// For any given `(previous_key_pressed, current_key_pressed)`:
/// - if `(current_key_pressed, None)` is in the map and the modifiers match `KeyMods`,
///   reset `previous_key_pressed = None` and invoke `Message`
/// - else if `(previous_key_pressed, Some(current_key_pressed))` is in the map, and the modifiers
///   match `KeyMods` invoke `Message` and reset `previous_key_pressed = None`
/// - else update `previous_key_pressed = Some(current_key_pressed)`
#[derive(Debug, Default)]
pub struct KeyMap {
    /// Map of Key Pressed => Action when pressing that key
    keys: HashMap<KeySequence, (KeyMods, KeyAction)>,
}

/// Keybindings for ferrishot
#[derive(knus::Decode, Debug, Default)]
pub struct Keys {
    /// A list of raw keybindings for ferrishot, directly as read from the config file
    #[knus(children)]
    pub keys: Vec<Key>,
}

impl FromIterator<Key> for KeyMap {
    fn from_iter<T: IntoIterator<Item = Key>>(iter: T) -> Self {
        Self {
            keys: iter.into_iter().map(Key::action).collect(),
        }
    }
}

/// A sequence of 2 keys. If there are 2 keys like so:
/// - (T, None)
/// - (T, Some(X))
///
/// The 2nd key will never be triggered.
/// We will first search the `HashMap` of keys for the first key.
/// If it does not exist, search for the 2nd key.
#[derive(Debug, Hash, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct KeySequence(pub (IcedKey<char>, Option<IcedKey<char>>));

/// Modifier keys
#[derive(Debug, Default, Clone)]
pub struct KeyMods(pub iced::keyboard::Modifiers);

impl FromStr for KeyMods {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mods = iced::keyboard::Modifiers::empty();
        if s.is_empty() {
            return Ok(Self(Modifiers::empty()));
        }
        for modifier_str in s.split('+') {
            let modifier = match modifier_str {
                "shift" => Modifiers::SHIFT,
                "ctrl" => Modifiers::CTRL,
                "alt" => Modifiers::ALT,
                "super" | "windows" | "command" => Modifiers::LOGO,
                invalid => return Err(format!("Invalid modifier: {invalid}")),
            };
            if mods.contains(modifier) {
                return Err(format!("Duplicate modifier: {modifier_str}"));
            }
            mods.insert(modifier);
        }

        Ok(Self(mods))
    }
}

impl std::str::FromStr for KeySequence {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut keys = vec![];
        // For example, a string like `<<` is valid and means
        // pressing the `<` key twice in a row.
        let mut maybe_parsing_named_key = false;
        let mut named_key_buf = String::new();
        let mut chars = s.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '<' {
                if maybe_parsing_named_key {
                    // we encounter the second `<` in a row
                    //
                    // <<
                    //  x <-- we are here
                    //
                    // that means
                    // the first one was 100% a key.
                    keys.push(IcedKey::Character('<'));

                    // SPECIAL-CASE: there is no next character, the user literally
                    // type a sequence of `<` like: `<<`. So make sure we include the current `<`
                    if chars.peek().is_none() {
                        keys.push(IcedKey::Character('<'));
                    }
                } else {
                    maybe_parsing_named_key = true;
                }
            } else if maybe_parsing_named_key {
                if ch == '>' {
                    if named_key_buf.is_empty() {
                        // SPECIAL-CASE: in this case the user types exactly `<>`
                        // Make sure that the first `<` is also not ignored
                        keys.push(IcedKey::Character('<'));
                        keys.push(IcedKey::Character('>'));
                    } else {
                        // we are currently at the end of a named key
                        //
                        // <space>
                        //       x <-- we are here
                        //
                        // it must be a valid key
                        keys.push(IcedKey::Named(
                            Named::from_str(&named_key_buf)
                                .map_err(|err| format!("Invalid key: <{named_key_buf}>. {err}"))?
                                .to_iced(),
                        ));
                        named_key_buf.clear();
                    }
                    maybe_parsing_named_key = false;
                } else {
                    // we are currently inside of a named key like so
                    //
                    // <space>
                    //   x <-- we may be here
                    named_key_buf.push(ch);
                }
            } else {
                keys.push(IcedKey::Character(ch));
            }
        }
        let mut keys = keys.into_iter();
        let Some(first_key) = keys.next() else {
            return Err(String::from("Expected at least 1 key."));
        };
        let second_key = keys.next();
        if keys.next().is_some() {
            // because we only store a single previous key, we can't handle keybindings
            // with more than 1 key. Since this is a screenshot app and not something like a
            // text editor, I don't believe there is much utility in allowing 3 keys in a row or more.
            //
            // This greatly simplifies the code, as we don't have to be generic.
            return Err(String::from(
                "At the moment, only up to 2 keys in a sequence are supported.",
            ));
        }
        Ok(Self((first_key, second_key)))
    }
}

#[cfg(test)]
mod test {
    use iced::keyboard::key;

    use super::*;

    #[test]
    fn parse_key_sequence() {
        use IcedKey::Character as Ch;
        use IcedKey::Named;
        use KeySequence as Seq;

        assert_eq!("gh".parse::<Seq>(), Ok(Seq((Ch('g'), Some(Ch('h'))))));
        assert_eq!("ge".parse::<Seq>(), Ok(Seq((Ch('g'), Some(Ch('e'))))));
        assert_eq!("x".parse::<Seq>(), Ok(Seq((Ch('x'), None))));
        assert_eq!("Lx".parse::<Seq>(), Ok(Seq((Ch('L'), Some(Ch('x'))))));
        assert_eq!(
            "".parse::<Seq>(),
            Err("Expected at least 1 key.".to_string())
        );
        assert_eq!(
            "<space>x".parse::<Seq>(),
            Ok(Seq((Named(key::Named::Space), Some(Ch('x')))))
        );
        assert_eq!(
            "<space><space>".parse::<Seq>(),
            Ok(Seq((
                Named(key::Named::Space),
                Some(Named(key::Named::Space))
            )))
        );
        assert_eq!(
            "x<space>".parse::<Seq>(),
            Ok(Seq((Ch('x'), Some(Named(key::Named::Space)))))
        );
        assert_eq!("<<".parse::<Seq>(), Ok(Seq((Ch('<'), Some(Ch('<'))))));
        assert_eq!("<>".parse::<Seq>(), Ok(Seq((Ch('<'), Some(Ch('>'))))));
        assert_eq!(">>".parse::<Seq>(), Ok(Seq((Ch('>'), Some(Ch('>'))))));
        assert_eq!(
            "<<space>".parse::<Seq>(),
            Ok(Seq((Ch('<'), Some(Named(key::Named::Space)))))
        );
        assert_eq!(
            "<f32><f31>".parse::<Seq>(),
            Ok(Seq((Named(key::Named::F32), Some(Named(key::Named::F31)))))
        );
        assert_eq!(
            "><f32>".parse::<Seq>(),
            Ok(Seq((Ch('>'), Some(Named(key::Named::F32)))))
        );
        assert_eq!(
            "abc".parse::<Seq>(),
            Err("At the moment, only up to 2 keys in a sequence are supported.".to_string())
        );
        assert_eq!(
            "<f32>b<f16>".parse::<Seq>(),
            Err("At the moment, only up to 2 keys in a sequence are supported.".to_string())
        );
        assert_eq!(
            "<@>".parse::<Seq>(),
            Err("Invalid key: <@>. Matching variant not found".to_string())
        );
    }
}
