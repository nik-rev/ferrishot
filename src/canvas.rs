//! The canvas handles drawing the selection frame
use iced::Event::{Keyboard, Mouse};
use iced::advanced::debug::core::SmolStr;
use iced::keyboard::Event::KeyPressed;
use iced::keyboard::Event::KeyReleased;
use iced::keyboard::Key::{self, Character, Named};
use iced::keyboard::Modifiers;
use iced::keyboard::Modifiers as Mods;
use iced::keyboard::key::Named::F11;
use iced::keyboard::key::Named::{Enter, Escape, Shift};
use iced::mouse::Button::{Left, Right};
use iced::mouse::Event::ButtonPressed;
use iced::mouse::Event::ButtonReleased;
use iced::mouse::Event::CursorMoved;
use iced::{
    Rectangle, Renderer, Theme,
    mouse::{self, Interaction},
    widget::{self, Action, canvas},
};

/// Holds information about the mouse
#[derive(Default, Debug, Clone)]
pub struct KeysState {
    /// Left mouse click is currently being held down
    is_left_down: bool,
    /// Left mouse click is currently being held down
    is_right_down: bool,
    /// Shift key is currently being held down
    is_shift_down: bool,
    /// The last key that was pressed
    last_key_pressed: Option<Key>,
}

use crate::CONFIG;
use crate::config::KeyAction;
use crate::config::key::{KeyMods, KeySequence};
use crate::selection::Speed;
use crate::{
    App,
    corners::SideOrCorner,
    message::Message,
    selection::{Selection, SelectionStatus, selection_lock::OptionalSelectionExt as _},
};

impl canvas::Program<Message> for App {
    type State = KeysState;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        self.render_shade(&mut frame, bounds);

        if let Some(selection) = self.selection.map(Selection::norm) {
            selection.render_border(&mut frame, CONFIG.theme.selection_frame);
            selection
                .corners()
                .render_circles(&mut frame, CONFIG.theme.selection_frame);
        }

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> iced::advanced::mouse::Interaction {
        self.selection
            // mouse button for resizing the selection
            .and_then(|sel| {
                // if we are already resizing, then this cursor takes priority
                // e.g. we are resizing horizontally but we are on the top left
                // corner = we should have horizontal resize cursor.
                (if let SelectionStatus::Resize { resize_side, .. } = sel.status {
                    Some(resize_side.mouse_icon())
                } else if sel.status.is_move() {
                    Some(Interaction::Grabbing)
                } else {
                    None
                })
                .or_else(|| {
                    // when we started dragging a side, even if we go outside of the bounds of that side (which
                    // happens often when we are dragging the mouse fast), we don't want the cursor to change
                    cursor.position().and_then(|cursor| {
                        sel.corners().side_at(cursor).map(SideOrCorner::mouse_icon)
                    })
                })
            })
            .unwrap_or_else(|| {
                if self.cursor_in_selection(cursor).is_some() {
                    Interaction::Grab
                } else {
                    Interaction::Crosshair
                }
            })
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: &iced::Event,
        _bounds: Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> Option<widget::Action<Message>> {
        // handle keybindings first
        if let Keyboard(KeyPressed {
            modifiers,
            text,
            key,
            ..
        }) = event
        {
            let mut modifiers = *modifiers;

            // Shift key does not matter. For example:
            // - pressing `<` and the `SHIFT` modifier will be pressed
            // - `G` will also trigger the `SHIFT` modifier
            modifiers.remove(Modifiers::SHIFT);

            let key = if let Key::Named(name) = key {
                // named key takes priority over anything.
                //
                // For example, if we input `Escape` it will send the `text`
                // `\u{1b}` which won't match any of the keys that we have. So we must
                // intercept before that happens
                Key::Named(*name)
            } else {
                // if we input `G` for example, it actually sends:
                // - modifier: `shift`
                // - key: `g`
                // - text: `G`
                //
                // if we input `<` it sends:
                // - modifier: `shift`
                // - key: `,`
                // - text: `<`
                //
                // So `text` is our source of truth. However, sometimes it is not available.
                // If `key != Key::Named` and `text == None` then we use the actual `key`
                // as a fallback.
                //
                // It is unknown when this fallback might be used, but it is kept just in case.
                text.as_ref()
                    .map_or_else(|| key.clone(), |ch| Key::Character(ch.clone()))
            };

            if let Some(action) = CONFIG
                .keys
                .keys
                // e.g. for instance keybind for `g` should take priority over `gg`
                .get(&(KeySequence((key.clone(), None)), KeyMods(modifiers)))
                // e.g. in this case we try the `gg` keybinding since `g` does not exist
                .or_else(|| {
                    state
                        .last_key_pressed
                        .as_ref()
                        .and_then(|last_key_pressed| {
                            CONFIG.keys.keys.get(&(
                                KeySequence((last_key_pressed.clone(), Some(key.clone()))),
                                KeyMods(modifiers),
                            ))
                        })
                })
            {
                // the last key pressed needs to be reset for it to be
                // correct in future invocations
                //
                // For example if I press `gg`, and it activates some keybinding
                // I would have to press `gg` *again* to active it.
                //
                // If we did not reset, then `ggg` would trigger the `gg` keybindings
                // twice
                state.last_key_pressed = None;
                return Some(Action::publish(Message::KeyBind(action.clone())));
            }

            // the "Shift" is already included in the modifiers
            //
            // This way pressing e.g. `G` would only set the last_key_pressed once
            if key != Named(Shift) {
                state.last_key_pressed = Some(key);
            }
        }

        // other events
        let message = match event {
            Mouse(ButtonPressed(Left)) => {
                state.is_left_down = true;
                Message::LeftMouseDown(cursor)
            }
            Mouse(ButtonPressed(Right)) => {
                state.is_right_down = true;
                if let Some(cursor) = cursor.position() {
                    if let Some((selection, sel_is_some)) = self.selection.get() {
                        Message::ResizeToCursor {
                            cursor_pos: cursor,
                            selection: selection.norm(),
                            sel_is_some,
                        }
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            Mouse(ButtonReleased(Right)) => {
                state.is_right_down = false;
                Message::EnterIdle
            }
            Mouse(ButtonReleased(Left)) => {
                state.is_left_down = false;
                if CONFIG.instant && self.selections_created == 1 {
                    // we have created 1 selections in total, (the current one),
                    // in which case we want to copy it to the clipboard as the
                    // --instant flag was provided
                    Message::KeyBind(KeyAction::CopyToClipboard)
                } else {
                    Message::EnterIdle
                }
            }
            Keyboard(KeyReleased {
                key: Named(Shift), ..
            }) => {
                state.is_shift_down = false;
                Message::NoOp
            }
            Keyboard(KeyPressed {
                key: Named(Shift), ..
            }) => {
                state.is_shift_down = true;

                // If we are already resizing a side, and we press shift, we
                // want to act as if we just started resizing from this point again
                // so we do not get a surprising jump
                if let Some((selection, sel_is_some)) = self.selection.get() {
                    cursor
                        .position()
                        .map_or(Message::NoOp, |current_cursor_pos| {
                            match selection.status {
                                SelectionStatus::Resize { resize_side, .. } => Message::Resize {
                                    resize_side,
                                    // start resizing from this point on
                                    current_cursor_pos,
                                    initial_cursor_pos: current_cursor_pos,
                                    // the current selection becomes the new starting point
                                    initial_rect: selection.rect,
                                    sel_is_some,
                                    speed: Speed::Slow {
                                        has_speed_changed: true,
                                    },
                                },
                                SelectionStatus::Move { .. } => Message::MoveSelection {
                                    current_cursor_pos,
                                    initial_cursor_pos: current_cursor_pos,
                                    current_selection: selection,
                                    initial_rect_pos: selection.pos(),
                                    speed: Speed::Slow {
                                        has_speed_changed: true,
                                    },
                                },
                                _ => Message::NoOp,
                            }
                        })
                } else {
                    Message::NoOp
                }
            }
            Mouse(CursorMoved { position }) if self.selection.is_some_and(Selection::is_resize) => {
                // FIXME: this will not be necessary when we have `let_chains`
                let (selection, sel_is_some) =
                    self.selection.get().expect("has `.is_some_and()` guard");

                // FIXME: this will not be necessary when we have `let_chains`
                let SelectionStatus::Resize {
                    resize_side,
                    initial_rect,
                    initial_cursor_pos,
                } = selection.status
                else {
                    unreachable!("has `.is_some_and(is_resized)` guard");
                };

                Message::Resize {
                    current_cursor_pos: *position,
                    resize_side,
                    initial_cursor_pos,
                    initial_rect,
                    sel_is_some,
                    speed: if state.is_shift_down {
                        Speed::Slow {
                            has_speed_changed: false,
                        }
                    } else {
                        Speed::Regular
                    },
                }
            }
            Mouse(CursorMoved { position }) if self.selection.is_some_and(Selection::is_move) => {
                // FIXME: this will not be necessary when we have `let_chains`
                let current_selection = self.selection.expect("has `.is_some_and()` guard").norm();

                // FIXME: this will not be necessary when we have `let_chains`
                let SelectionStatus::Move {
                    initial_rect_pos,
                    initial_cursor_pos,
                } = current_selection.status
                else {
                    unreachable!();
                };

                Message::MoveSelection {
                    current_cursor_pos: *position,
                    initial_cursor_pos,
                    current_selection,
                    initial_rect_pos,
                    speed: if state.is_shift_down {
                        Speed::Slow {
                            has_speed_changed: false,
                        }
                    } else {
                        Speed::Regular
                    },
                }
            }
            Mouse(CursorMoved { position }) if self.selection.is_some_and(Selection::is_create) => {
                Message::ExtendNewSelection(*position)
            }
            _ => return None,
        };

        log::info!("Received message: {message:#?}");

        Some(Action::publish(message))
    }
}
