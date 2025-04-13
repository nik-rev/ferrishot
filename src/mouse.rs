//! Mouse state - left clicks and such

/// Holds information about the mouse
#[derive(Default, Debug, Clone, Copy)]
pub struct MouseState {
    /// Left mouse click is currently being held down
    is_left_down: bool,
    /// Left mouse click is currently being held down
    is_right_down: bool,
}

impl MouseState {
    /// Register a left mouse click
    pub const fn left_click(&mut self) {
        self.is_left_down = true;
    }

    /// Left mouse button is not clicked
    pub const fn left_release(&mut self) {
        self.is_left_down = false;
    }

    /// If the left mouse button is clicked
    pub const fn is_left_clicked(self) -> bool {
        self.is_left_down
    }

    /// If the left mouse button is released
    pub const fn is_left_released(self) -> bool {
        !self.is_left_down
    }
    /// Register a left mouse click
    pub const fn right_click(&mut self) {
        self.is_right_down = true;
    }

    /// Right mouse button is not clicked
    pub const fn right_release(&mut self) {
        self.is_right_down = false;
    }

    /// If the right mouse button is clicked
    pub const fn is_right_clicked(self) -> bool {
        self.is_right_down
    }

    /// If the right mouse button is released
    pub const fn is_right_released(self) -> bool {
        !self.is_right_down
    }
}
