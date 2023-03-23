mod input_control;
mod sandbox;

const DISPLAY_WIDTH: u32 = (sandbox::SIMULATION_WIDTH * 2) as u32;
const DISPLAY_HEIGHT: u32 = (sandbox::SIMULATION_HEIGHT * 2) as u32;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sandbox = sandbox::Sandbox::new();

    let event_loop = winit::event_loop::EventLoop::new();
    let window = {
        let size = winit::dpi::LogicalSize::new(DISPLAY_WIDTH as f64, DISPLAY_HEIGHT as f64);
        println!("set windows logical size {:?}", size);
        winit::window::WindowBuilder::new()
            .with_title("Sandbox")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)?
    };
    let mut pixels = {
        let window_size = window.inner_size();
        println!("get window physical size {:?}", window_size);
        let surface_texture =
            pixels::SurfaceTexture::new(window_size.width, window_size.height, &window);
        pixels::Pixels::new(
            sandbox::SIMULATION_WIDTH as u32,
            sandbox::SIMULATION_HEIGHT as u32,
            surface_texture,
        )?
    };
    pixels.set_clear_color(pixels::wgpu::Color::WHITE);

    let mut input_control = input_control::InputControl::new();

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::MainEventsCleared => {
            if input_control.is_pressed() {
                let (x, y) = input_control.last_cursor_pos();
                let x = (x / 2.0) as usize;
                let y = (y / 2.0) as usize;
                sandbox.cells[x][y] = Some(sandbox::Particle::new());
            }
            sandbox.update();
            window.request_redraw();
        }
        winit::event::Event::RedrawRequested(_) => {
            sandbox.render(pixels.get_frame_mut());
            pixels.render().unwrap();
        }
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit
            }
            winit::event::WindowEvent::CursorMoved { position, .. } => {
                input_control.cursor_move(position);
            }
            winit::event::WindowEvent::MouseInput { button, state, .. } => {
                if button == winit::event::MouseButton::Left {
                    match state {
                        winit::event::ElementState::Pressed => input_control.press(),
                        winit::event::ElementState::Released => input_control.release(),
                    }
                }
            }
            _ => {}
        },
        _ => {}
    });
}
