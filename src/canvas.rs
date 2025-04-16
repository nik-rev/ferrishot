//! The canvas handles drawing the selection frame
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
}

use crate::{
    App, CONFIG,
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
                // when we started dragging a side, even if we go outside of the bounds of that side (which
                // happens often when we are dragging the mouse fast), we don't want the cursor to change
                cursor
                    .position()
                    .and_then(|cursor| sel.corners().side_at(cursor).map(SideOrCorner::mouse_icon))
                    // for example, if we start dragging top right corner, and move mouse to the
                    // top left corner, we want the cursor to switch appropriately
                    .or_else(|| {
                        if let SelectionStatus::Resize { resize_side, .. } = sel.status {
                            Some(resize_side.mouse_icon())
                        } else {
                            None
                        }
                    })
            })
            .unwrap_or_else(|| {
                if self.selection.is_some_and(Selection::is_move) {
                    Interaction::Grabbing
                } else if self.cursor_in_selection(cursor).is_some() {
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
        use iced::Event::Mouse;

        let message = match event {
            Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                state.is_left_down = true;
                Message::LeftMouseDown(cursor)
            }
            Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) => {
                state.is_right_down = true;
                if let Some(cursor) = cursor.position() {
                    if let Some((selection, sel_is_some)) = self.selection.get() {
                        Message::ResizingToCursor {
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
            Mouse(mouse::Event::ButtonReleased(mouse::Button::Right)) => {
                state.is_right_down = false;
                Message::EnterIdle
            }
            Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                state.is_left_down = false;
                if CONFIG.instant && self.selections_created == 1 {
                    Message::CopyToClipboard
                } else {
                    Message::EnterIdle
                }
            }
            Mouse(mouse::Event::CursorMoved { position })
                if self.selection.is_some_and(Selection::is_resize) =>
            {
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

                Message::InitialResize {
                    current_cursor_pos: *position,
                    resize_side,
                    initial_cursor_pos,
                    initial_rect,
                    sel_is_some,
                }
            }
            Mouse(mouse::Event::CursorMoved { position })
                if self.selection.is_some_and(Selection::is_move) =>
            {
                // FIXME: this will not be necessary when we have `let_chains`
                let current_selection = self.selection.expect("has `.is_some_and()` guard");

                // FIXME: this will not be necessary when we have `let_chains`
                let SelectionStatus::Move {
                    initial_rect_pos,
                    initial_cursor_pos,
                } = current_selection.status
                else {
                    unreachable!();
                };

                Message::MovingSelection {
                    current_cursor_pos: *position,
                    initial_cursor_pos,
                    current_selection,
                    initial_rect_pos,
                }
            }
            Mouse(mouse::Event::CursorMoved { position })
                if self.selection.is_some_and(Selection::is_create) =>
            {
                Message::ExtendNewSelection(*position)
            }
            Mouse(mouse::Event::ButtonPressed(mouse::Button::Middle)) => Message::FullSelection,
            _ => return None,
        };

        log::info!("Received message: {message:#?}");

        Some(Action::publish(message))
    }
}
