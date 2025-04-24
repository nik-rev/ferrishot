//! Main logic for the application, handling of events and mutation of the state

use crate::CONFIG;
use crate::config::KeyAction;
use crate::config::Place;
use crate::selection::Speed;
use crate::widget::Letters;
use crate::widget::PickCorner;
use crate::widget::WelcomeMessage;
use std::borrow::Cow;
use std::time::Instant;

use crate::message::Message;
use crate::screenshot::RgbaHandle;
use crate::selection::selection_lock::OptionalSelectionExt;
use iced::alignment::Vertical;
use iced::mouse::Cursor;
use iced::widget::canvas::Path;
use iced::widget::text::Shaping;
use iced::widget::{self, Column, Space, Stack, canvas, column, container, row};
use iced::{Background, Color, Element, Font, Length, Point, Rectangle, Size, Task};

use crate::corners::{Direction, Side, SideOrCorner};
use crate::rectangle::RectangleExt;
use crate::selection::{Selection, SelectionStatus};

/// The image to save to a file, chosen by the user in a file picker.
///
/// Unfortunately, there is simply no way to communicate something from
/// the inside of an iced application to the outside: i.e. "Return" something
/// from an iced program exiting. So we have to use a global variable for this.
///
/// This global is mutated just *once* at the end of the application's lifetime,
/// when the window closes.
///
/// It is then accessed just *once* to open the file dialog and let the user pick
/// where they want to save their image.
///
/// Yes, at the moment we want this when using Ctrl + S to save as file:
/// 1. Close the application to save the file and generate the image we'll save
/// 2. Open the file explorer, and save the image to the specified path
///
/// When the file explorer is spawned from the inside of an iced window, closing
/// this window will then also close the file explorer. It means that we can't
/// close the window and then spawn an explorer.
///
/// The other option is to have both windows open at the same time. But this
/// would be really odd. First of all, we will need to un-fullscreen the App
/// because the file explorer can spawn under the app.
///
/// This is going to be sub-optimal. Currently, we give off the illusion of
/// drawing shapes and rectangles on top of the desktop. It is not immediately
/// obvious that the app is just another window which is full-screen.
/// Doing the above would break that illusion.
///
/// Ideally, we would be able to spawn a file explorer *above* the window without
/// having to close this. But this seems to not be possible. Perhaps in the
/// future there will be some kind of file explorer Iced widget that we
/// can use instead of the native file explorer.
pub static SAVED_IMAGE: std::sync::OnceLock<image::DynamicImage> = std::sync::OnceLock::new();

/// Show an error message to the user
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct ErrorMessage {
    /// Error message
    message: Cow<'static, str>,
    /// When the error was created
    timestamp: Instant,
}

impl ErrorMessage {
    /// Create a new error message
    pub fn new<T: Into<Cow<'static, str>>>(message: T) -> Self {
        Self {
            message: message.into(),
            timestamp: Instant::now(),
        }
    }
}

/// Holds the state for ferrishot
#[derive(Debug)]
pub struct App {
    /// How many selections were created throughout the
    /// lifetime of the App
    pub selections_created: usize,
    /// The full screenshot of the monitor from which ferrishot was invoked
    /// We then create a window spanning the entire monitor, with this
    /// screenshot as background, with a canvas rendered on top - giving the
    /// illusion that we are drawing shapes on top of the screen.
    pub image: RgbaHandle,
    /// Area of the screen that is selected for capture
    pub selection: Option<Selection>,
    /// Errors to display to the user
    pub errors: Vec<ErrorMessage>,
    /// Shows a grid of letters on the screen, pressing 3 letters in a row
    /// allows accessing 25 * 25 * 25 = 15,625 different locations
    pub picking_corner: Option<PickCorner>,
    /// A link to the uploaded image
    pub uploaded_url: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        let screenshot =
            crate::screenshot::screenshot().expect("Failed to take a screenshot of the desktop");

        Self {
            image: screenshot,
            selection: None,
            selections_created: 0,
            errors: vec![],
            picking_corner: None,
            uploaded_url: None,
        }
    }
}

impl App {
    /// Renders the app
    pub fn view(&self) -> iced::Element<Message> {
        Stack::new()
            // taken screenshot in the background
            .push(crate::widget::BackgroundImage {
                image_handle: RgbaHandle::clone(&self.image).into(),
            })
            // border around the selection
            .push(canvas(self).width(Length::Fill).height(Length::Fill))
            // information popup, when there is no selection
            .push_maybe(self.selection.is_none().then(|| {
                crate::widget::WelcomeMessage {
                    image_width: self.image.width(),
                    image_height: self.image.height(),
                }
                .view()
            }))
            // errors
            .push(self.render_errors())
            // icons around the selection
            .push_maybe(self.selection.filter(|sel| sel.is_idle()).map(|selection| {
                crate::widget::Icons {
                    image_width: self.image.width() as f32,
                    image_height: self.image.height() as f32,
                    selection_rect: selection.rect.norm(),
                }
                .view()
            }))
            // grid of letters to precisely choose a location
            .push_maybe(self.picking_corner.as_ref().map(|letters| {
                crate::widget::Letters {
                    pick_corner: *letters,
                }
                .view()
            }))
            // size indicator
            .push_maybe(self.selection.filter(|_| CONFIG.size_indicator).get().map(
                |(sel, sel_is_some)| {
                    crate::widget::SizeIndicator {
                        image_height: self.image.height(),
                        image_width: self.image.width(),
                        selection_rect: sel.norm().rect,
                        sel_is_some,
                    }
                    .view()
                },
            ))
            .into()
    }

    /// Modifies the app's state
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LettersAbort => {
                self.picking_corner = None;
            }
            Message::LettersPick { point } => {
                let sel = self.selection.map(Selection::norm).unwrap_or_default();
                let x = point.x;
                let y = point.y;
                if let Some(pick_corner) = self.picking_corner {
                    match pick_corner {
                        PickCorner::TopLeft => {
                            self.selection = Some(sel.with_x(|_| x).with_y(|_| y));
                        }
                        PickCorner::BottomRight => {
                            self.selection = Some(
                                sel.with_height(|_| y - sel.rect.y)
                                    .with_width(|_| x - sel.rect.x),
                            );
                        }
                    }
                };
                self.picking_corner = None;
            }
            Message::ResizeVertically {
                new_height,
                sel_is_some,
            } => {
                let sel = self.selection.unlock(sel_is_some);

                // what is the minimum value for `new_height` that would make
                // this overflow vertically?
                // We want to make sure the selection cannot get bigger than that.
                let new_height =
                    new_height.min((sel.norm().rect.y + sel.norm().rect.height) as u32);

                let dy = new_height as f32 - sel.norm().rect.height;
                *sel = sel
                    .norm()
                    .with_height(|_| new_height as f32)
                    .with_y(|y| y - dy);
            }
            Message::ResizeHorizontally {
                new_width,
                sel_is_some,
            } => {
                let sel = self.selection.unlock(sel_is_some);

                // what is the minimum value for `new_width` that would make
                // this overflow vertically?
                // We want to make sure the selection cannot get bigger than that.
                let new_width = new_width.min((sel.norm().rect.x + sel.norm().rect.width) as u32);

                let dx = new_width as f32 - sel.norm().rect.width;
                *sel = sel
                    .norm()
                    .with_width(|_| new_width as f32)
                    .with_x(|x| x - dx);
            }
            Message::NoOp => (),
            Message::LeftMouseDown(cursor) => {
                if let Some((cursor, side, rect)) = cursor.position().and_then(|cursor_pos| {
                    self.selection.as_mut().and_then(|selected_region| {
                        selected_region
                            .corners()
                            .side_at(cursor_pos)
                            .map(|l| (cursor_pos, l, selected_region))
                    })
                }) {
                    let resized = SelectionStatus::Resize {
                        initial_rect: rect.norm().rect,
                        initial_cursor_pos: cursor,
                        resize_side: side,
                    };
                    rect.status = resized;
                } else if let Some((cursor, selected_region)) = self.cursor_in_selection_mut(cursor)
                {
                    let dragged = SelectionStatus::Move {
                        initial_rect_pos: selected_region.norm().pos(),
                        initial_cursor_pos: cursor,
                    };
                    selected_region.status = dragged;
                } else if let Some(cursor_position) = cursor.position() {
                    // no region is selected, select the initial region
                    self.create_selection_at(cursor_position);
                }
            }
            Message::EnterIdle => {
                if let Some(selection) = self.selection.as_mut() {
                    selection.status = SelectionStatus::Idle;
                }
            }
            Message::MoveSelection {
                current_cursor_pos,
                initial_cursor_pos,
                current_selection,
                initial_rect_pos,
                speed,
            } => {
                let (image_width, image_height, _) = self.image.raw();
                let mut new_selection = current_selection.with_pos(|_| {
                    initial_rect_pos + ((current_cursor_pos - initial_cursor_pos) * speed.speed())
                });

                let old_x = new_selection.rect.x as u32;
                let old_y = new_selection.rect.y as u32;

                // if any of these actually get changed we are going to set the new selection status.

                new_selection.rect.x = new_selection
                    .rect
                    .x
                    .min(image_width as f32 - new_selection.rect.width)
                    .max(0.0);

                new_selection.rect.y = new_selection
                    .rect
                    .y
                    .min(image_height as f32 - new_selection.rect.height)
                    .max(0.0);

                if new_selection.rect.y as u32 != old_y || new_selection.rect.x as u32 != old_x {
                    new_selection.status = SelectionStatus::Move {
                        initial_rect_pos: new_selection.pos(),
                        initial_cursor_pos: current_cursor_pos,
                    }
                }

                if speed
                    == (Speed::Slow {
                        has_speed_changed: true,
                    })
                {
                    new_selection.status = SelectionStatus::Move {
                        initial_rect_pos: current_selection.pos(),
                        initial_cursor_pos: current_cursor_pos,
                    }
                }

                self.selection = Some(new_selection);
            }
            Message::KeyBind { action, count } => match action {
                KeyAction::ClearSelection => {
                    self.selection = None;
                }
                KeyAction::SelectFullScreen => {
                    let (width, height, _) = self.image.raw();
                    {
                        self.selection = Some(Selection::new(Point { x: 0.0, y: 0.0 }).with_size(
                            |_| Size {
                                width: width as f32,
                                height: height as f32,
                            },
                        ));
                    }
                }
                KeyAction::CopyToClipboard => {
                    let Some(selection) = self.selection.map(Selection::norm) else {
                        self.error("There is no selection to copy");
                        return Task::none();
                    };

                    let (width, height, pixels) = self.image.raw();

                    let cropped_image = selection.process_image(width, height, pixels);

                    let image_data = arboard::ImageData {
                        width: cropped_image.width() as usize,
                        height: cropped_image.height() as usize,
                        bytes: std::borrow::Cow::Borrowed(cropped_image.as_bytes()),
                    };

                    #[cfg_attr(
                        target_os = "macos",
                        expect(unused_variables, reason = "it is used on other platforms")
                    )]
                    match crate::clipboard::set_image(image_data) {
                        Ok(img_path) => {
                            // send desktop notification if possible, this is
                            // just a decoration though so it's ok if we fail to do this
                            let mut notify = notify_rust::Notification::new();

                            notify.summary(&format!(
                                "Copied image to clipboard {width}px * {height}px"
                            ));

                            // images are not supported on macos
                            #[cfg(not(target_os = "macos"))]
                            notify.image_path(&img_path.to_string_lossy());

                            let _ = notify.show();

                            return Self::exit();
                        }
                        Err(err) => {
                            self.error(format!("Could not copy the image: {err}"));
                        }
                    }
                }
                KeyAction::SaveScreenshot => {
                    let Some(selection) = self.selection.as_ref().map(|sel| Selection::norm(*sel))
                    else {
                        self.error("Selection does not exist. There is nothing to copy!");
                        return Task::none();
                    };

                    let (width, height, pixels) = self.image.raw();
                    let cropped_image = selection.process_image(width, height, pixels);

                    let _ = SAVED_IMAGE.set(cropped_image);

                    return Self::exit();
                }
                KeyAction::Exit => return Self::exit(),
                KeyAction::SetWidth => {
                    let Some(selection) = self.selection.as_mut() else {
                        self.error("Nothing is selected.");
                        return Task::none();
                    };
                    let (image_width, _, _) = self.image.raw();
                    let image_width = image_width as f32;
                    let sel = selection.norm();

                    *selection = sel.with_width(|_| (count as f32).min(image_width - sel.rect.x));
                }
                KeyAction::SetHeight => {
                    let Some(selection) = self.selection.as_mut() else {
                        self.error("Nothing is selected.");
                        return Task::none();
                    };
                    let (_, image_height, _) = self.image.raw();
                    let image_height = image_height as f32;
                    let sel = selection.norm();

                    *selection = sel.with_height(|_| (count as f32).min(image_height - sel.rect.y));
                }
                KeyAction::Goto(place) => {
                    let Some(selection) = self.selection.as_mut() else {
                        self.error("Nothing is selected.");
                        return Task::none();
                    };
                    let (image_width, image_height, _) = self.image.raw();
                    let image_height = image_height as f32;
                    let image_width = image_width as f32;
                    let sel = selection.norm();

                    *selection = match place {
                        Place::Center => sel
                            .with_x(|_| (image_width - sel.rect.width) / 2.0)
                            .with_y(|_| (image_height - sel.rect.height) / 2.0),
                        Place::XCenter => sel.with_x(|_| (image_width - sel.rect.width) / 2.0),
                        Place::YCenter => sel.with_y(|_| (image_height - sel.rect.height) / 2.0),
                        Place::TopLeft => sel.with_x(|_| 0.0).with_y(|_| 0.0),
                        Place::TopRight => {
                            sel.with_x(|_| image_width - sel.rect.width).with_y(|_| 0.0)
                        }
                        Place::BottomLeft => sel
                            .with_x(|_| 0.0)
                            .with_y(|_| image_height - sel.rect.height),
                        Place::BottomRight => sel
                            .with_x(|_| image_width - sel.rect.width)
                            .with_y(|_| image_height - sel.rect.height),
                        Place::Top => sel.with_y(|_| 0.0),
                        Place::Bottom => sel.with_y(|_| image_height - sel.rect.height),
                        Place::Left => sel.with_x(|_| 0.0),
                        Place::Right => sel.with_x(|_| image_width - sel.rect.width),
                    }
                }
                KeyAction::Move(direction, amount) => {
                    let Some(selection) = self.selection.as_mut() else {
                        self.error("Nothing is selected.");
                        return Task::none();
                    };
                    let (image_width, image_height, _) = self.image.raw();
                    let image_height = image_height as f32;
                    let image_width = image_width as f32;
                    let amount = amount as f32 * count as f32;
                    let sel = selection.norm();

                    *selection = match direction {
                        Direction::Up => sel.with_y(|y| (y - amount).max(0.0)),
                        Direction::Down => {
                            sel.with_y(|y| (y + amount).min(image_height - sel.rect.height))
                        }
                        Direction::Left => sel.with_x(|x| (x - amount).max(0.0)),
                        Direction::Right => {
                            sel.with_x(|x| (x + amount).min(image_width - sel.rect.width))
                        }
                    }
                }
                KeyAction::Extend(direction, amount) => {
                    let Some(selection) = self.selection.as_mut() else {
                        self.error("Nothing is selected.");
                        return Task::none();
                    };
                    let (image_width, image_height, _) = self.image.raw();
                    let image_height = image_height as f32;
                    let image_width = image_width as f32;
                    let sel = selection.norm();
                    let amount = amount as f32 * count as f32;

                    *selection = match direction {
                        Direction::Up => sel
                            .with_y(|y| (y - amount).max(0.0))
                            .with_height(|h| (h + amount).min(sel.rect.y + sel.rect.height)),
                        Direction::Down => {
                            sel.with_height(|h| (h + amount).min(image_height - sel.rect.y))
                        }
                        Direction::Left => sel
                            .with_x(|x| (x - amount).max(0.0))
                            .with_width(|w| (w + amount).min(sel.rect.x + sel.rect.width)),
                        Direction::Right => {
                            sel.with_width(|w| (w + amount).min(image_width - sel.rect.x))
                        }
                    }
                }
                KeyAction::Shrink(direction, amount) => {
                    let Some(selection) = self.selection.as_mut() else {
                        self.error("Nothing is selected.");
                        return Task::none();
                    };
                    let sel = selection.norm();
                    let amount = amount as f32 * count as f32;

                    *selection = match direction {
                        Direction::Up => sel
                            .with_y(|y| (y + amount).min(sel.rect.y + sel.rect.height))
                            .with_height(|h| (h - amount).max(0.0)),
                        Direction::Down => sel.with_height(|h| (h - amount).max(0.0)),
                        Direction::Left => sel
                            .with_x(|x| (x + amount).min(sel.rect.x + sel.rect.width))
                            .with_width(|w| (w - amount).max(0.0)),
                        Direction::Right => sel.with_width(|w| (w - amount).max(0.0)),
                    }
                }
                KeyAction::PickTopLeftCorner => {
                    self.picking_corner = Some(PickCorner::TopLeft);
                }
                KeyAction::PickBottomRightCorner => {
                    self.picking_corner = Some(PickCorner::BottomRight);
                }
            },
            Message::ExtendNewSelection(new_mouse_position) => {
                self.update_selection(new_mouse_position);
            }
            Message::Upload => {
                let Some(selection) = self.selection.as_ref().map(|sel| Selection::norm(*sel))
                else {
                    self.error("Selection does not exist. There is nothing to copy!");
                    return Task::none();
                };

                let (width, height, pixels) = self.image.raw();
                let cropped_image = selection.process_image(width, height, pixels);
                let tempfile = match tempfile::TempDir::new() {
                    Ok(tempdir) => tempdir.into_path().join("ferrishot-screenshot.png"),
                    Err(err) => {
                        self.error(err.to_string());
                        return Task::none();
                    }
                };

                if let Err(err) = cropped_image.save_with_format(&tempfile, image::ImageFormat::Png)
                {
                    self.error(err.to_string());
                }

                return Task::future(async move {
                    {
                        let file = tempfile;
                        let response = CONFIG
                            .default_image_upload_provider
                            .upload_image(&file)
                            .await;

                        match response {
                            Ok(url) => Message::ImageUploaded { url },
                            Err(err) => Message::Error(err.to_string()),
                        }
                    }
                });
            }
            Message::ImageUploaded { url } => {
                self.uploaded_url = Some(url);
            }
            Message::ExitImageUploadMenu => self.uploaded_url = None,
            Message::Error(err) => {
                self.error(err);
            }
            Message::Resize {
                current_cursor_pos,
                initial_cursor_pos,
                resize_side,
                initial_rect,
                sel_is_some,
                speed,
            } => {
                let selected_region = self.selection.unlock(sel_is_some);
                let resize_speed = speed.speed();

                let dy = (current_cursor_pos.y - initial_cursor_pos.y) * resize_speed;
                let dx = (current_cursor_pos.x - initial_cursor_pos.x) * resize_speed;

                selected_region.rect = match resize_side {
                    SideOrCorner::Side(side) => match side {
                        Side::Top => initial_rect.with_height(|h| h - dy).with_y(|y| y + dy),
                        Side::Right => initial_rect.with_width(|w| w + dx),
                        Side::Bottom => initial_rect.with_height(|h| h + dy),
                        Side::Left => initial_rect.with_width(|w| w - dx).with_x(|x| x + dx),
                    },
                    SideOrCorner::Corner(corner) => corner.resize_rect(initial_rect, dy, dx),
                };

                if speed
                    == (Speed::Slow {
                        has_speed_changed: true,
                    })
                {
                    selected_region.status = SelectionStatus::Resize {
                        initial_rect: selected_region.rect,
                        initial_cursor_pos: current_cursor_pos,
                        resize_side,
                    }
                }
            }
            Message::ResizeToCursor {
                cursor_pos,
                selection,
                sel_is_some,
            } => {
                let (corner_point, corners) = selection.corners().nearest_corner(cursor_pos);
                let sel = self.selection.unlock(sel_is_some);

                sel.rect = corners.resize_rect(
                    selection.rect,
                    cursor_pos.y - corner_point.y,
                    cursor_pos.x - corner_point.x,
                );

                sel.status = SelectionStatus::Resize {
                    initial_rect: sel.rect,
                    initial_cursor_pos: cursor_pos,
                    resize_side: SideOrCorner::Corner(corners),
                };
            }
        }

        Task::none()
    }

    /// Renders the black tint on regions that are not selected
    pub fn draw_shade(&self, frame: &mut canvas::Frame, bounds: Rectangle) {
        let Some(selection) = self.selection.map(Selection::norm) else {
            // fill the entire screen in non-selected
            frame.fill_rectangle(
                bounds.position(),
                bounds.size(),
                CONFIG.theme.non_selected_region,
            );
            return;
        };

        // represents the area outside of the selection
        let outside = Path::new(|p| {
            p.move_to(bounds.top_left());
            p.line_to(bounds.top_right());
            p.line_to(bounds.bottom_right());
            p.line_to(bounds.bottom_left());
            p.move_to(bounds.top_left());
            p.move_to(selection.top_left());
            p.line_to(selection.bottom_left());
            p.line_to(selection.bottom_right());
            p.line_to(selection.top_right());
            p.move_to(selection.top_left());
        });

        frame.fill(&outside, CONFIG.theme.non_selected_region);
    }

    /// Shows up to 3 of the most recent errors in the UI
    fn render_errors(&self) -> iced::Element<Message> {
        const ERROR_WIDTH: u32 = 300;

        let errors = self
            .errors()
            .into_iter()
            // don't display more than the most recent 3 errors
            .take(3)
            .map(|error| {
                container(widget::text!("Error: {error}"))
                    .height(80)
                    .width(ERROR_WIDTH)
                    .style(|_| container::Style {
                        text_color: Some(CONFIG.theme.error_fg),
                        background: Some(Background::Color(CONFIG.theme.error_bg)),
                        border: iced::Border {
                            color: CONFIG.theme.drop_shadow,
                            width: 4.0,
                            radius: 2.0.into(),
                        },
                        shadow: iced::Shadow::default(),
                    })
                    .padding(10.0)
                    .into()
            })
            .collect::<Column<_>>()
            .width(ERROR_WIDTH)
            .spacing(30);

        let (image_width, _, _) = self.image.raw();

        row![Space::with_width(image_width - ERROR_WIDTH), errors].into()
    }

    /// If the given cursor intersects the selected region, give the region and
    /// the cursor
    pub fn cursor_in_selection(&self, cursor: Cursor) -> Option<(Point, Selection)> {
        self.selection.and_then(|sel| {
            cursor
                .position()
                .and_then(|cursor_pos| sel.contains(cursor_pos).then_some((cursor_pos, sel)))
        })
    }
    /// If the given cursor intersects the selected region, give the region and
    /// the cursor
    fn cursor_in_selection_mut(&mut self, cursor: Cursor) -> Option<(Point, &mut Selection)> {
        self.selection.as_mut().and_then(|sel| {
            cursor
                .position()
                .and_then(|cursor_pos| sel.norm().contains(cursor_pos).then_some((cursor_pos, sel)))
        })
    }

    /// Create an empty selection at the current position
    pub fn create_selection_at(&mut self, create_selection_at: Point) {
        let mut selection = Selection::new(create_selection_at);
        selection.status = SelectionStatus::Create;
        self.selections_created += 1;
        self.selection = Some(selection);
    }

    /// Computes a new selection based on the current position
    pub fn update_selection(&mut self, other: Point) {
        self.selection = self.selection.take().map(|selected_region| {
            #[rustfmt::skip]
            {
    // selected_region -> x1y1-------------------------x2
    //   (fixed)          |             ^
    //                    |           width            ~
    //                    |
    //                    |
    //                    | <- height                  ~
    //                    |
    //                    |                            ~
    //                    |
    //                   y2    ~      ~       ~   ~  x2y2 <- create_selection_at (can move)
            };
            let width = other.x - selected_region.rect.x;
            let height = other.y - selected_region.rect.y;

            selected_region.with_size(|_| Size { width, height })
        });
    }

    /// Close the app
    ///
    /// This is like `iced::exit`, but it does not cause a segfault in special
    /// circumstances <https://github.com/iced-rs/iced/issues/2625>
    fn exit() -> Task<Message> {
        iced::window::get_latest().then(|id| iced::window::close(id.expect("window to exist")))
    }

    /// Add a new error to the list of errors
    fn error<T: Into<Cow<'static, str>> + std::fmt::Display>(&mut self, error: T) {
        log::error!("Status Error: {error}");
        self.errors.push(ErrorMessage::new(error));
    }

    /// Retrieve errors that have not yet expired
    fn errors(&self) -> Vec<String> {
        /// When there is an error, display it for this amount of time
        const ERROR_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(5);

        let now = Instant::now();
        self.errors
            .iter()
            .rev()
            .map_while(|err| {
                let time_passed = now - err.timestamp;
                (time_passed <= ERROR_TIMEOUT).then_some(err.message.to_string())
            })
            .collect()
    }
}
