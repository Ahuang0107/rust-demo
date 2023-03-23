pub struct InputControl {
    prev_cursor_position: winit::dpi::PhysicalPosition<f64>,
    curr_cursor_position: winit::dpi::PhysicalPosition<f64>,
    pressed: bool,
}

impl InputControl {
    pub fn new() -> Self {
        Self {
            prev_cursor_position: winit::dpi::PhysicalPosition::<f64>::new(0.0, 0.0),
            curr_cursor_position: winit::dpi::PhysicalPosition::<f64>::new(0.0, 0.0),
            pressed: false,
        }
    }
    pub fn press(&mut self) {
        self.pressed = true;
    }
    pub fn release(&mut self) {
        self.pressed = false;
    }
    pub fn is_pressed(&self) -> bool {
        self.pressed
    }
    pub fn cursor_move(&mut self, pos: winit::dpi::PhysicalPosition<f64>) {
        self.prev_cursor_position = self.curr_cursor_position;
        self.curr_cursor_position = pos;
    }
    pub fn last_cursor_pos(&self) -> (f64, f64) {
        (self.curr_cursor_position.x, self.curr_cursor_position.y)
    }
}
