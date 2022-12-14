use wgpu::util::DeviceExt;

pub struct Context {
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: crate::rectangle::Vec2,
    shader: wgpu::ShaderModule,
    triangle_render_pipeline: wgpu::RenderPipeline,
    line_render_pipeline: wgpu::RenderPipeline,
    glyph_brush: wgpu_glyph::GlyphBrush<()>,
}

impl Context {
    pub async fn try_new(canvas: &web_sys::HtmlCanvasElement) -> anyhow::Result<Self> {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = instance.create_surface_from_canvas(canvas);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(anyhow::anyhow!("unable to get adapter"))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    limits: wgpu::Limits::downlevel_webgl2_defaults(),
                },
                None, // Trace path
            )
            .await?;

        let render_format = surface.get_supported_formats(&adapter)[0];

        let (width, height) = (canvas.width(), canvas.height());
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: render_format,
            width,
            height,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let triangle_render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                    // or Features::POLYGON_MODE_POINT, which is native only
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                // If the pipeline will be used with a multiview render pass, this
                // indicates how many array layers the attachments will have.
                multiview: None,
            });

        let line_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
                // or Features::POLYGON_MODE_POINT
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        let inconsolata =
            wgpu_glyph::ab_glyph::FontArc::try_from_slice(include_bytes!("MSYHMONO.ttf"))?;

        let glyph_brush =
            wgpu_glyph::GlyphBrushBuilder::using_font(inconsolata).build(&device, render_format);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size: crate::rectangle::Vec2::from(width as f32, height as f32),
            shader,
            triangle_render_pipeline,
            line_render_pipeline,
            glyph_brush,
        })
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
        // TODO 这里的position是齐次坐标系下的x,y,z
        //  x,y在(-1.0,1.0)范围内
        //  z在(0,1.0)范围内
        //  具体看 https://www.w3.org/TR/webgpu/#coordinate-systems
        //  需要具体看如何根据world transform和canvas size来判断最终得到的output position是什么
        // 生成直线的数据
        // let (line_vertices, line_indices) = crate::rectangle::Line {
        //     pos1: crate::rectangle::Vec2::from(32.0, 0.0),
        //     pos2: crate::rectangle::Vec2::from(32.0, self.size.y),
        // }
        // .to_vertices_and_indices(crate::rectangle::Vec2::from(self.size.x, self.size.y));
        let mut lines = crate::rectangle::Lines::new();
        for xi in 0..40 {
            lines.lines.push(crate::rectangle::Line {
                pos1: crate::rectangle::Vec2::from(xi as f32 * 32.0, 0.0),
                pos2: crate::rectangle::Vec2::from(xi as f32 * 32.0, self.size.y),
            });
        }
        let (line_vertices, line_indices) =
            lines.to_vertices_and_indices(&crate::rectangle::Vec2::from(self.size.x, self.size.y));
        let line_vertex_buffer =
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(&line_vertices[0..]),
                    usage: wgpu::BufferUsages::VERTEX,
                });
        let line_index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&line_indices[0..]),
                usage: wgpu::BufferUsages::INDEX,
            });
        let line_num_indices = line_indices.len() as u32;
        // 生成正方形的数据
        let (vertices, indices) = crate::rectangle::Rectangle::from_square(250.0, 250.0, 50.0)
            .to_vertices_and_indices(crate::rectangle::Vec2::from(self.size.x, self.size.y));

        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });
        let num_indices = indices.len() as u32;

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
            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            pass.draw_indexed(0..num_indices, 0, 0..1);
        }

        let mut staging_belt = wgpu::util::StagingBelt::new(1024);
        self.glyph_brush.queue(wgpu_glyph::Section {
            screen_position: (30.0, 90.0),
            bounds: (self.size.x, self.size.y),
            text: vec![wgpu_glyph::Text::new(
                "Hello 中文繁體!add 更多文字内容检查文字渲染是否正常",
            )
            .with_color([0.0, 0.0, 0.0, 1.0])
            .with_scale(12.0)],
            ..wgpu_glyph::Section::default()
        });
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

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
    pub fn mock(x: f32, y: f32) -> Self {
        Self {
            position: [x, y, 0.0],
            color: [0.0, 0.0, 0.0],
        }
    }
}
