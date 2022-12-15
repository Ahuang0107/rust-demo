pub struct RgbColor {
    r: u16,
    g: u16,
    b: u16,
}

impl RgbColor {
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const RO: Self = Self {
        r: 12,
        g: 12,
        b: 12,
    };
    pub fn from(r: u16, g: u16, b: u16) -> Self {
        Self { r, g, b }
    }
    pub fn to_rgb_slice(&self) -> [f32; 3] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        ]
    }
    pub fn to_rgba_slice(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            1.0,
        ]
    }
}
