#[derive(Default)]
pub struct Size {
    pub x: f32,
    pub y: f32,
}

impl Size {
    pub fn from(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
