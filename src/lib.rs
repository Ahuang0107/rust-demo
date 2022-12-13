use wasm_bindgen::prelude::*;
use wgpu::util::DeviceExt;

mod context;
mod dom;

#[wasm_bindgen]
pub struct Canvas {
    dom: web_sys::HtmlCanvasElement,
    context: context::Context,
}

#[wasm_bindgen]
impl Canvas {
    pub async fn new() -> Self {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        let dom = dom::get_or_create_canvas("wasm-example");
        let context = context::Context::try_new(&dom).await.unwrap();
        Self { dom, context }
    }
    pub fn run(&mut self, x_offset: f32, y_offset: f32) {
        let left_top = (x_offset, y_offset);
        let vertices: &[context::Vertex] = &[
            context::Vertex {
                position: [-0.1 + left_top.0, 0.1 + left_top.1, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // A
            context::Vertex {
                position: [-0.1 + left_top.0, -0.1 + left_top.1, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // B
            context::Vertex {
                position: [0.1 + left_top.0, -0.1 + left_top.1, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // C
            context::Vertex {
                position: [0.1 + left_top.0, 0.1 + left_top.1, 0.0],
                color: [0.5, 0.0, 0.5],
            }, // D
        ];

        let indices: &[u16] = &[0, 1, 3, 1, 2, 3];

        let vertex_buffer =
            self.context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });
        let index_buffer =
            self.context
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                });
        let num_indices = indices.len() as u32;
        // let start = instant::Instant::now();
        self.context
            .render(vertex_buffer, index_buffer, num_indices)
            .expect("TODO: panic message");
        // let end = instant::Instant::now().duration_since(start).as_millis();
        // gloo_console::log!(format!("render cost: {}", end))
    }
}
