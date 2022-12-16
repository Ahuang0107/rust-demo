use wgpu::util::DeviceExt;
use wgpu::Buffer;

use crate::color::RgbColor;
use crate::gpu::vertex::Vertex;

use super::point::Point;
use super::size::Size;

#[derive(Default)]
pub struct Rectangle {
    pub position: Point,
    pub size: Size,
}

impl Rectangle {
    pub const INDICES: [u16; 6] = [0, 1, 3, 1, 2, 3];
    pub fn from(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            position: Point::from(x, y),
            size: Size::from(width, height),
        }
    }
    fn left_top(&self) -> Point {
        Point::from(self.position.x, self.position.y)
    }
    fn left_bottom(&self) -> Point {
        Point::from(self.position.x, self.position.y + self.size.y)
    }
    fn right_top(&self) -> Point {
        Point::from(self.position.x + self.size.x, self.position.y)
    }
    fn right_bottom(&self) -> Point {
        Point::from(self.position.x + self.size.x, self.position.y + self.size.y)
    }
    pub fn to_vertices_and_indices(&self, ndc_size: &Size, color: &RgbColor) -> [Vertex; 4] {
        let vertices: [Vertex; 4] = [
            Vertex::from(&self.left_top(), ndc_size, color),
            Vertex::from(&self.left_bottom(), ndc_size, color),
            Vertex::from(&self.right_bottom(), ndc_size, color),
            Vertex::from(&self.right_top(), ndc_size, color),
        ];
        vertices
    }
}

#[derive(Default)]
pub struct Rectangles {
    pub rectangles: Vec<Rectangle>,
}

impl Rectangles {
    fn to_vertices_and_indices(
        &self,
        ndc_size: &Size,
        color: &RgbColor,
    ) -> (Vec<Vertex>, Vec<u16>) {
        let mut rect_vertices: Vec<Vertex> = vec![];
        let mut rect_indices: Vec<u16> = vec![];
        self.rectangles.iter().for_each(|rect| {
            // 注意这里每次的indices需要加上的是rect_vertices的数量而不是rect_indices的数量
            let indices_len = rect_vertices.len();
            let vertices: [Vertex; 4] = rect.to_vertices_and_indices(ndc_size, color);
            vertices.into_iter().for_each(|v| rect_vertices.push(v));
            Rectangle::INDICES
                .into_iter()
                .map(|i| i + indices_len as u16)
                .for_each(|i| rect_indices.push(i));
        });
        (rect_vertices, rect_indices)
    }
    pub fn to_buffers_info(
        &self,
        device: &wgpu::Device,
        ndc_size: &Size,
        color: &RgbColor,
    ) -> (Buffer, Buffer, u32) {
        let (rect_vertices, rect_indices) = self.to_vertices_and_indices(ndc_size, color);
        let rect_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&rect_vertices[0..]),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let rect_index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&rect_indices[0..]),
            usage: wgpu::BufferUsages::INDEX,
        });
        let rect_num_indices = rect_indices.len() as u32;
        (rect_vertex_buffer, rect_index_buffer, rect_num_indices)
    }
}
