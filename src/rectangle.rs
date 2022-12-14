pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn from(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct Rectangle {
    position: Vec2,
    size: Vec2,
}

impl Rectangle {
    pub fn square(x: f32, y: f32, size: f32) -> Self {
        Self {
            position: Vec2::from(x, y),
            size: Vec2::from(size, size),
        }
    }
    fn left_top(&self) -> Vec2 {
        Vec2 {
            x: self.position.x,
            y: self.position.y,
        }
    }
    fn left_bottom(&self) -> Vec2 {
        Vec2 {
            x: self.position.x,
            y: self.position.y + self.size.y,
        }
    }
    fn right_top(&self) -> Vec2 {
        Vec2 {
            x: self.position.x + self.size.x,
            y: self.position.y,
        }
    }
    fn right_bottom(&self) -> Vec2 {
        Vec2 {
            x: self.position.x + self.size.x,
            y: self.position.y + self.size.y,
        }
    }
    pub fn to_vertices_and_indices(
        &self,
        screen_space_size: Vec2,
    ) -> ([crate::context::Vertex; 4], [u16; 6]) {
        let vertices: [crate::context::Vertex; 4] = [
            crate::context::Vertex {
                position: [
                    (self.left_top().x / screen_space_size.x) * 2.0 - 1.0,
                    1.0 - (self.left_top().y / screen_space_size.y) * 2.0,
                    0.0,
                ],
                color: [0.5, 0.0, 0.5],
            }, // A
            crate::context::Vertex {
                position: [
                    (self.left_bottom().x / screen_space_size.x) * 2.0 - 1.0,
                    1.0 - (self.left_bottom().y / screen_space_size.y) * 2.0,
                    0.0,
                ],
                color: [0.5, 0.0, 0.5],
            }, // B
            crate::context::Vertex {
                position: [
                    (self.right_bottom().x / screen_space_size.x) * 2.0 - 1.0,
                    1.0 - (self.right_bottom().y / screen_space_size.y) * 2.0,
                    0.0,
                ],
                color: [0.5, 0.0, 0.5],
            }, // C
            crate::context::Vertex {
                position: [
                    (self.right_top().x / screen_space_size.x) * 2.0 - 1.0,
                    1.0 - (self.right_top().y / screen_space_size.y) * 2.0,
                    0.0,
                ],
                color: [0.5, 0.0, 0.5],
            }, // D
        ];
        let indices: [u16; 6] = [0, 1, 3, 1, 2, 3];
        (vertices, indices)
    }
}
