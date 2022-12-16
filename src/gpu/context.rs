use anyhow::anyhow;
use gloo_console::log;
use instant::Instant;
use wgpu::ShaderSource::Wgsl;
use wgpu::{
    Backends, Device, DeviceDescriptor, Features, Instance, Limits, PowerPreference,
    PrimitiveTopology, Queue, RenderPipeline, RequestAdapterOptions, ShaderModuleDescriptor,
    Surface, SurfaceConfiguration,
};
use wgpu_glyph::ab_glyph::FontArc;
use wgpu_glyph::{GlyphBrush, GlyphBrushBuilder, Section};

use crate::color::RgbColor;
use crate::geometry::line::*;
use crate::geometry::rectangle::*;
use crate::geometry::size::Size;
use crate::gpu::pipeline::create_render_pipeline;

#[allow(dead_code)]
pub struct Context {
    surface: Surface,
    pub device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    pub size: Size,
    triangle_render_pipeline: RenderPipeline,
    line_render_pipeline: RenderPipeline,
    glyph_brush: GlyphBrush<()>,
    lines: Lines,
    rects: Rectangles,
    sections: Vec<Section<'static>>,
}

impl Context {
    pub async fn try_new(canvas: &web_sys::HtmlCanvasElement) -> anyhow::Result<Self> {
        let instance = Instance::new(Backends::all());
        let surface = instance.create_surface_from_canvas(canvas);

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(anyhow!("unable to get adapter"))?;

        let supported_formats = surface.get_supported_formats(&adapter);
        let render_format = supported_formats[0];

        log!(format!("supported_formats: {:?}", supported_formats));
        log!(format!("adapter info: {:?}", adapter.get_info()));
        log!(format!(
            "downlevel_capabilities: {:?}",
            adapter.get_downlevel_capabilities()
        ));
        log!(format!(
            "texture_format_features: {:?}",
            adapter.get_texture_format_features(render_format)
        ));

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    features: Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: Limits::downlevel_webgl2_defaults(),
                },
                None, // Trace path
            )
            .await?;

        let (width, height) = (canvas.client_width() as u32, canvas.client_height() as u32);
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: render_format,
            width,
            height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &config);

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: Wgsl(include_str!("shader/shader.wgsl").into()),
        });

        let triangle_render_pipeline =
            create_render_pipeline(&device, &shader, &config, PrimitiveTopology::TriangleList);

        let line_render_pipeline =
            create_render_pipeline(&device, &shader, &config, PrimitiveTopology::LineList);

        let font = FontArc::try_from_slice(include_bytes!("../assets/Montserrat Regular.otf"))?;

        let glyph_brush = GlyphBrushBuilder::using_font(font).build(&device, render_format);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size: Size::from(width as f32, height as f32),
            triangle_render_pipeline,
            line_render_pipeline,
            glyph_brush,
            lines: Lines::default(),
            rects: Rectangles::default(),
            sections: vec![],
        })
    }

    pub fn render_with_cost(&mut self) -> anyhow::Result<()> {
        let start = Instant::now();
        let result = self.render();
        log!(format!(
            "cost: {}ms",
            Instant::now().duration_since(start).as_millis()
        ));
        result
    }

    pub fn push_line(&mut self, line: Line) {
        self.lines.lines.push(line)
    }

    pub fn push_rect(&mut self, rect: Rectangle) {
        self.rects.rectangles.push(rect)
    }

    pub fn push_section(&mut self, section: Section<'static>) {
        self.sections.push(section);
    }

    pub fn render(&mut self) -> anyhow::Result<()> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let (line_vertex_buffer, line_index_buffer, line_num_indices) =
            self.lines
                .to_buffers_info(&self.device, &self.size, &RgbColor::RO);

        let (rect_vertex_buffer, rect_index_buffer, rect_num_indices) =
            self.rects
                .to_buffers_info(&self.device, &self.size, &RgbColor::BLACK);

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            pass.set_pipeline(&self.line_render_pipeline);
            pass.set_vertex_buffer(0, line_vertex_buffer.slice(..));
            pass.set_index_buffer(line_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(0..line_num_indices, 0, 0..1);

            pass.set_pipeline(&self.triangle_render_pipeline);
            pass.set_vertex_buffer(0, rect_vertex_buffer.slice(..));
            pass.set_index_buffer(rect_index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(0..rect_num_indices, 0, 0..1);
        }

        let mut staging_belt = wgpu::util::StagingBelt::new(1024);

        self.sections.iter().for_each(|s| self.glyph_brush.queue(s));
        self.glyph_brush
            .draw_queued(
                &self.device,
                &mut staging_belt,
                &mut encoder,
                &view,
                self.size.x as u32,
                self.size.y as u32,
            )
            .expect("Draw queued");
        staging_belt.finish();

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        staging_belt.recall();

        Ok(())
    }
}
