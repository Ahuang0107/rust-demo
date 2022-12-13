use wasm_bindgen::JsCast;

pub fn get_or_create_canvas(id: &str) -> web_sys::HtmlCanvasElement {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| {
            let foreign = d.get_element_by_id(id)?;
            let canvas = d.create_element("canvas").expect("fail to create <canvas>");
            foreign.append_child(&canvas).ok()?;
            Some(canvas.unchecked_into::<web_sys::HtmlCanvasElement>())
        })
        .expect("fail to append <canvas>")
}
