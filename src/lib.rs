use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod context;

#[wasm_bindgen]
pub struct Runtime {
    context: context::Context,
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
        // TODO 现在又一个问题就是画布绘制的内容会被拉伸，这个问题具体看nbody-wasm-sim是如何解决的
        canvas
            .set_attribute("width", rect.width().to_string().as_str())
            .unwrap();
        canvas
            .set_attribute("height", rect.height().to_string().as_str())
            .unwrap();
        foreign.append_child(&canvas).ok();

        let context = context::Context::try_new(&canvas).await.unwrap();
        Self { context }
    }
    pub fn render(mut self) {
        self.context.render();
    }
}
