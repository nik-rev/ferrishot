//! Grid allows to position items on a canvas in a grid with labels

use bon::Builder;
use iced::{
    Point, Rectangle, Size,
    advanced::graphics::geometry,
    widget::canvas::{self, Frame, Stroke},
};

use crate::rect::{PointExt, RectangleExt, StrokeExt, TextExt};

/// A cell in a grid
#[derive(Clone, Debug, Builder)]
pub struct Cell<'frame, Draw: FnOnce(&mut Frame, Rectangle)> {
    /// The closure. Determines what to draw inside of the table cell
    draw: Draw,
    /// Stroke to draw around the cell
    stroke: Option<Stroke<'frame>>,
    /// Label of the cell. Drawn above the cell
    label: Option<geometry::Text>,
    /// Description of the cell. Drawn below the cell
    description: Option<geometry::Text>,
}

impl<Draw: FnOnce(&mut Frame, Rectangle)> Cell<'_, Draw> {
    /// Draw the `Cell`
    pub fn draw(self, frame: &mut canvas::Frame, bounds: Rectangle) {
        // Stroke
        if let Some(stroke) = self.stroke {
            frame.stroke_rectangle(bounds.top_left(), bounds.size(), stroke);
        }

        // Label
        if let Some(label) = self.label {
            // center horizontally
            let label = label.position(|text_size| {
                Point::new(
                    bounds.center_x_for(text_size),
                    bounds.y - text_size.height - 4.0,
                )
            });

            frame.fill_text(label);
        }

        // Description
        if let Some(description) = self.description {
            let description = description.position(|text_size| {
                Point::new(
                    bounds.center_x_for(text_size),
                    bounds.y + bounds.height + 4.0,
                )
            });

            frame.fill_text(description);
        }

        // Draw cell contents
        (self.draw)(frame, bounds);
    }
}

/// A grid for a canvas
#[derive(Clone, Debug, Builder)]
pub struct Grid<'frame, Draw: FnOnce(&mut Frame, Rectangle)> {
    /// Top-left corner of the `Grid`
    top_left: Point,
    /// Cells of the grid
    cells: Vec<Cell<'frame, Draw>>,
    /// Column count of the grid
    columns: usize,
    /// Size of each item
    cell_size: Size,
    /// Title of the grid. Drawn above the grid
    title: Option<geometry::Text>,
    /// Description of the grid. Drawn below the grid
    description: Option<geometry::Text>,
    /// Labels of the rows
    row_labels: Option<Vec<String>>,
    /// Labels of the columns
    col_labels: Option<Vec<String>>,
    /// Draw red border around grid items, for debugging purposes
    #[builder(default, with = || true)]
    dbg: bool,
    /// How much space to put between each item
    spacing: Size,
}

impl<Draw: FnOnce(&mut Frame, Rectangle)> Grid<'_, Draw> {
    /// Vertical margin of the title
    const TITLE_VMARGIN: f32 = 8.0;

    /// Size of the `Grid`
    pub fn size(&self) -> Size {
        let rows = self.cells.len() / self.columns;

        Size {
            width: (self.columns as f32).mul_add(
                self.cell_size.width,
                (self.columns as f32 - 1.0) * self.spacing.width,
            ),
            height: (rows as f32).mul_add(
                self.cell_size.height,
                (rows as f32 - 1.0) * self.spacing.height,
            ) + self
                .title
                .as_ref()
                .map_or(0.0, |title| title.size().height + Self::TITLE_VMARGIN),
        }
    }

    /// Draw the `Grid` on the `Frame` of a `Canvas`
    pub fn draw(self, frame: &mut canvas::Frame) {
        let grid_rect = Rectangle::new(self.top_left, self.size());

        if self.dbg {
            frame.stroke_rectangle(grid_rect.top_left(), grid_rect.size(), Stroke::RED);
        }

        let title_height = self.title.map_or(0.0, |title| {
            let title_size = title.size();

            let title = title.position(|text_size| Point {
                x: grid_rect.center_x_for(text_size),
                y: grid_rect.y - text_size.height - Self::TITLE_VMARGIN,
            });

            if self.dbg {
                frame.stroke_rectangle(title.position, title_size, Stroke::RED);
            }

            frame.fill_text(title);

            title_size.height + Self::TITLE_VMARGIN
        });

        for (index, cell) in self.cells.into_iter().enumerate() {
            let rows_drawn = index / self.columns;
            let cols_drawn = index % self.columns;

            let cell_top_left = Point::new(
                (cols_drawn as f32).mul_add(
                    self.cell_size.width,
                    self.spacing.width * cols_drawn as f32 + -1.0,
                ),
                (rows_drawn as f32).mul_add(
                    self.cell_size.height,
                    self.spacing.height * rows_drawn as f32 + -1.0,
                ) + title_height,
            ) + self.top_left.into_vector();

            cell.draw(frame, Rectangle::new(cell_top_left, self.cell_size));

            if self.dbg {
                frame.stroke_rectangle(cell_top_left, self.cell_size, Stroke::RED);
            }
        }
    }
}
