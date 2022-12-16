use wgpu::util::DeviceExt;
use wgpu::Buffer;

use crate::color::RgbColor;
use crate::geometry::point::Point;
use crate::geometry::size::Size;
use crate::gpu::vertex::Vertex;

pub struct Line {
    pub pos1: Point,
    pub pos2: Point,
}

impl Line {
    pub fn from(pos1: Point, pos2: Point) -> Self {
        Self { pos1, pos2 }
    }
    pub fn to_vertices_and_indices(&self, ndc_size: &Size, color: &RgbColor) -> [Vertex; 2] {
        let vertices: [Vertex; 2] = [
            Vertex::from(&self.pos1, &ndc_size, color),
            Vertex::from(&self.pos2, &ndc_size, color),
        ];
        vertices
    }
}

#[derive(Default)]
pub struct Lines {
    pub lines: Vec<Line>,
}

impl Lines {
    fn to_vertices_and_indices(
        &self,
        ndc_size: &Size,
        color: &RgbColor,
    ) -> (Vec<Vertex>, Vec<u16>) {
        let mut line_vertices: Vec<Vertex> = vec![];
        let mut line_indices: Vec<u16> = vec![];
        self.lines.iter().for_each(|line| {
            let vertices: [Vertex; 2] = line.to_vertices_and_indices(ndc_size, color);
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
        ndc_size: &Size,
        color: &RgbColor,
    ) -> (Buffer, Buffer, u32) {
        let (line_vertices, line_indices) = self.to_vertices_and_indices(ndc_size, color);
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
