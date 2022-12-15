use std::fmt::format;
use std::ops::Deref;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod color;
mod context;
mod model;
mod rectangle;

#[wasm_bindgen]
pub struct Runtime {
    context: std::sync::Arc<std::sync::Mutex<context::Context>>,
}

#[wasm_bindgen]
impl Runtime {
    pub async fn new(foreign: web_sys::Element) -> Self {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        let document: web_sys::Document = web_sys::window().and_then(|w| w.document()).unwrap();
        let canvas: web_sys::HtmlCanvasElement = document
            .create_element("canvas")
            .expect("fail to create <canvas>")
            .unchecked_into::<web_sys::HtmlCanvasElement>();
        let rect = foreign.get_bounding_client_rect();
        canvas.set_width(rect.width() as u32);
        canvas.set_height(rect.height() as u32);
        foreign.append_child(&canvas).ok();

        let context = context::Context::try_new(&canvas).await.unwrap();
        let context = std::sync::Arc::new(std::sync::Mutex::new(context));

        {
            let context = context.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| {
                let mut context = context.lock().unwrap();
                context.render_with_cost();
            });
            canvas
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
        {
            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::WheelEvent| {
                gloo_console::log!(format!(
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

        context.lock().unwrap().render();

        Self { context }
    }
    pub fn push_data(&self) {}
}
