use std::mem::size_of;

use wgpu::{BufferAddress, VertexAttribute, VertexBufferLayout, VertexFormat};

use crate::color::RgbColor;
use crate::geometry::point::Point;
use crate::geometry::size::Size;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

#[allow(dead_code)]
impl Vertex {
    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: size_of::<Vertex>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Float32x3,
                },
                VertexAttribute {
                    offset: size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Float32x3,
                },
            ],
        }
    }
    /// will convert to Normalized Device Coordinate (NDC)
    /// 具体看 https://www.w3.org/TR/webgpu/#coordinate-systems
    pub fn from(point: &Point, ndc_size: &Size, color: &RgbColor) -> Self {
        Self {
            position: [
                (point.x / ndc_size.x) * 2.0 - 1.0,
                1.0 - (point.y / ndc_size.y) * 2.0,
                0.0,
            ],
            color: color.to_rgb_slice(),
        }
    }
}
