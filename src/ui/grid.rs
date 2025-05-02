//! Grid allows to position items on a canvas in a grid with labels

use derive_builder::Builder;
use iced::{
    Point, Rectangle, Size,
    advanced::graphics::geometry,
    widget::canvas::{self, Frame, Stroke},
};

use crate::rect::{PointExt, RectangleExt, TextExt as _};

/// A cell in a grid
#[derive(Clone, Debug, Builder)]
#[builder(setter(strip_option))]
pub struct Cell<'frame, Draw: FnOnce(&mut Frame, Rectangle)> {
    /// The closure. Determines what to draw inside of the table cell
    draw: Draw,
    /// Stroke to draw around the cell
    #[builder(default)]
    stroke: Option<Stroke<'frame>>,
    /// Label of the cell
    #[builder(default)]
    label: Option<geometry::Text>,
    /// Description of the cell
    #[builder(default)]
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
    #[builder(setter(each = "add_cell"))]
    cells: Vec<Cell<'frame, Draw>>,
    /// Column count of the grid
    columns: usize,
    /// Size of each item
    cell_size: Size,
    /// How much space to put between each item
    spacing: Size,
}

impl<Draw: FnOnce(&mut Frame, Rectangle)> Grid<'_, Draw> {
    /// Draw the `Grid` on the `Frame` of a `Canvas`
    pub fn draw(self, frame: &mut canvas::Frame) {
        for (index, cell) in self.cells.into_iter().enumerate() {
            let row_index = index / self.columns;
            let col_index = index % self.columns;

            let cell_top_left = Point::new(
                (col_index as f32).mul_add(
                    self.cell_size.width,
                    self.spacing.width.mul_add(col_index as f32, -1.0),
                ),
                (row_index as f32).mul_add(
                    self.cell_size.height,
                    self.spacing.height.mul_add(row_index as f32, -1.0),
                ),
            ) + self.top_left.into_vector();

            cell.draw(frame, Rectangle::new(cell_top_left, self.cell_size));
        }
    }
}
