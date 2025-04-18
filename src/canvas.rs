//! The canvas handles drawing the selection frame
use iced::Event::{Keyboard, Mouse};
use iced::keyboard::Event::KeyPressed;
use iced::keyboard::Event::KeyReleased;
use iced::keyboard::Key::{Character, Named};
use iced::keyboard::Modifiers as Mods;
use iced::keyboard::key::Named::F11;
use iced::keyboard::key::Named::{Enter, Escape, Shift};
use iced::mouse::Button::{Left, Middle, Right};
use iced::mouse::Event::ButtonPressed;
use iced::mouse::Event::ButtonReleased;
use iced::mouse::Event::CursorMoved;
use iced::{
    Rectangle, Renderer, Theme,
    mouse::{self, Interaction},
    widget::{self, Action, canvas},
};

/// Holds information about the mouse
#[derive(Default, Debug, Clone, Copy)]
pub struct MouseState {
    /// Left mouse click is currently being held down
    is_left_down: bool,
    /// Left mouse click is currently being held down
    is_right_down: bool,
    /// Shift key is currently being held down
    is_shift_down: bool,
}

use crate::selection::Speed;
use crate::{
    App,
    corners::SideOrCorner,
    message::Message,
    selection::{Selection, SelectionStatus, selection_lock::OptionalSelectionExt as _},
    theme::THEME,
};

impl canvas::Program<Message> for App {
    type State = MouseState;

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
            selection.render_border(&mut frame, THEME.accent);
            selection.corners().render_circles(&mut frame, THEME.accent);
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
                if self.config.instant && self.selections_created == 1 {
                    // we have created 1 selections in total, (the current one),
                    // in which case we want to copy it to the clipboard as the
                    // --instant flag was provided
                    Message::CopyToClipboard
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
            // Esc
            Keyboard(KeyPressed {
                key: Named(Escape), ..
            }) => Message::Exit,
            // Ctrl + C
            Keyboard(KeyPressed {
                key: Character(c),
                modifiers: Mods::CTRL,
                ..
            }) if *c == "c" => Message::CopyToClipboard,
            // Enter
            Keyboard(KeyPressed {
                key: Named(Enter), ..
            }) => Message::CopyToClipboard,
            // Ctrl + S
            Keyboard(KeyPressed {
                key: Character(c),
                modifiers: Mods::CTRL,
                ..
            }) if *c == "s" => Message::SaveScreenshot,
            Keyboard(KeyPressed {
                key: Named(F11), ..
            })
            | Mouse(ButtonPressed(Middle)) => Message::SelectFullScreen,
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
