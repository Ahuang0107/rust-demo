use wgpu::util::DeviceExt;

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
    pub fn from(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Vec2::from(x, y),
            size: Vec2::from(width, height),
        }
    }
    pub fn from_square(x: f32, y: f32, size: f32) -> Self {
        Self::from(x, y, size, size)
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
        color: &crate::color::RgbColor,
    ) -> ([crate::context::Vertex; 4], [u16; 6]) {
        let vertices: [crate::context::Vertex; 4] = [
            crate::context::Vertex::mock(
                (self.left_top().x / screen_space_size.x) * 2.0 - 1.0,
                1.0 - (self.left_top().y / screen_space_size.y) * 2.0,
                color,
            ),
            crate::context::Vertex::mock(
                (self.left_bottom().x / screen_space_size.x) * 2.0 - 1.0,
                1.0 - (self.left_bottom().y / screen_space_size.y) * 2.0,
                color,
            ),
            crate::context::Vertex::mock(
                (self.right_bottom().x / screen_space_size.x) * 2.0 - 1.0,
                1.0 - (self.right_bottom().y / screen_space_size.y) * 2.0,
                color,
            ),
            crate::context::Vertex::mock(
                (self.right_top().x / screen_space_size.x) * 2.0 - 1.0,
                1.0 - (self.right_top().y / screen_space_size.y) * 2.0,
                color,
            ),
        ];
        let indices: [u16; 6] = [0, 1, 3, 1, 2, 3];
        (vertices, indices)
    }
}

pub struct Line {
    pub pos1: Vec2,
    pub pos2: Vec2,
}

impl Line {
    pub fn to_vertices_and_indices(
        &self,
        screen_space_size: &Vec2,
        color: &crate::color::RgbColor,
    ) -> [crate::context::Vertex; 2] {
        let vertices: [crate::context::Vertex; 2] = [
            crate::context::Vertex::mock(
                (self.pos1.x / screen_space_size.x) * 2.0 - 1.0,
                1.0 - (self.pos1.y / screen_space_size.y) * 2.0,
                color,
            ),
            crate::context::Vertex::mock(
                (self.pos2.x / screen_space_size.x) * 2.0 - 1.0,
                1.0 - (self.pos2.y / screen_space_size.y) * 2.0,
                color,
            ),
        ];
        vertices
    }
}

pub struct Lines {
    pub lines: Vec<Line>,
}

impl Lines {
    pub fn new() -> Self {
        Self { lines: vec![] }
    }
    pub fn to_vertices_and_indices(
        &self,
        screen_space_size: &Vec2,
        color: &crate::color::RgbColor,
    ) -> (Vec<crate::context::Vertex>, Vec<u16>) {
        let mut line_vertices: Vec<crate::context::Vertex> = vec![];
        let mut line_indices: Vec<u16> = vec![];
        self.lines.iter().for_each(|line| {
            let vertices = line.to_vertices_and_indices(&screen_space_size, color);
            vertices.into_iter().for_each(|v| {
                line_vertices.push(v);
                line_indices.push(line_indices.len() as u16);
            });
        });
        (line_vertices, line_indices)
    }
    pub fn to_buffers_info(
        &self,
        device: &wgpu::Device,
        screen_space_size: &Vec2,
        color: &crate::color::RgbColor,
    ) -> (wgpu::Buffer, wgpu::Buffer, u32) {
        let (line_vertices, line_indices) = self.to_vertices_and_indices(screen_space_size, color);
        let line_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&line_vertices[0..]),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let line_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&line_indices[0..]),
            usage: wgpu::BufferUsages::INDEX,
        });
        let line_num_indices = line_indices.len() as u32;
        (line_vertex_buffer, line_index_buffer, line_num_indices)
    }
}
