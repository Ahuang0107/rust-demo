use std::panic::set_hook;
use std::sync::{Arc, Mutex};

use console_error_panic_hook::hook;
use gloo_console::log;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Document, HtmlCanvasElement, HtmlElement};
use wgpu_glyph::{Layout, Section, Text};

use crate::color::RgbColor;
use crate::geometry::line::Line;
use crate::geometry::point::Point;
use crate::geometry::rectangle::Rectangle;
use crate::gpu::context::Context;

#[wasm_bindgen]
#[allow(dead_code)]
pub struct Runtime {
    context: Arc<Mutex<Context>>,
}

#[wasm_bindgen]
impl Runtime {
    pub async fn new() -> Self {
        set_hook(Box::new(hook));
        let document: Document = window().and_then(|w| w.document()).unwrap();
        let body: HtmlElement = document.body().unwrap();
        let canvas: HtmlCanvasElement = document
            .create_element("canvas")
            .expect("fail to create <canvas>")
            .unchecked_into::<HtmlCanvasElement>();
        let rect = body.get_bounding_client_rect();
        canvas.set_width(rect.width() as u32);
        canvas.set_height(rect.height() as u32);
        body.append_child(&canvas).ok();

        let context = Context::try_new(&canvas).await.unwrap();
        let context = Arc::new(Mutex::new(context));

        {
            let context = context.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| {
                let mut context = context.lock().unwrap();
                context.render_with_cost().expect("");
            });
            canvas
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
        {
            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::WheelEvent| {
                log!(format!(
                    "wheel event(delta_x:{:?},delta_y:{:?})",
                    event.delta_x(),
                    event.delta_y()
                ))
            });
            canvas
                .add_event_listener_with_callback("wheel", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }

        {
            let mut context = context.lock().unwrap();
            let width = context.size.x;
            let height = context.size.y;
            for x in (0..context.size.x as u32).step_by(32) {
                context.push_line(Line::from(
                    Point::from(x as f32, 0.0),
                    Point::from(x as f32, height),
                ));
            }
            for y in (0..context.size.y as u32).step_by(24) {
                context.push_line(Line {
                    pos1: Point::from(0.0, y as f32),
                    pos2: Point::from(width, y as f32),
                });
            }
        }
        {
            let mut context = context.lock().unwrap();
            let width = context.size.x;
            let height = context.size.y;
            for x in (0..width as u32).step_by(64) {
                for y in (0..height as u32).step_by(48) {
                    context.push_rect(Rectangle::from(x as f32, y as f32, 32.0, 24.0))
                }
            }
        }
        {
            let mut context = context.lock().unwrap();
            let width = context.size.x;
            let height = context.size.y;
            for x in (0..width as u32).step_by(64) {
                for y in (0..height as u32).step_by(48) {
                    context.push_section(Section {
                        screen_position: (x as f32, y as f32),
                        bounds: (32.0, 24.0),
                        layout: Layout::default_single_line(),
                        text: vec![Text::new("Sphinx Of Black Quartz, Judge My Vow.")
                            .with_color(RgbColor::RO.to_rgba_slice())
                            .with_scale(14.0)],
                        ..Section::default()
                    })
                }
            }
        }

        context.lock().unwrap().render().expect("");

        Self { context }
    }
}
