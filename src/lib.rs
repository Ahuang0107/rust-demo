use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn hello(str: &str) {
    console_log!("Hello {}!", str);
}

#[wasm_bindgen]
pub fn pass_params_cost_test(params: JsValue) {
    gloo_console::log!(format!(
        "received params({})",
        std::mem::size_of_val(&params)
    ))
}

#[wasm_bindgen]
pub struct Matrix(Vec<Vec<f64>>);

#[wasm_bindgen]
impl Matrix {
    pub fn from_array(array: JsValue) -> Self {
        let start = instant::Instant::now();
        let array = serde_wasm_bindgen::from_value::<serde_json::Value>(array).unwrap();
        let mut result: Vec<Vec<f64>> = vec![];
        if let Some(cols) = array.as_array() {
            cols.into_iter().for_each(|col| {
                if let Some(col) = col.as_array() {
                    result.push(
                        col.into_iter()
                            .map(|v| v.as_f64().unwrap())
                            .collect::<Vec<f64>>(),
                    );
                }
            });
        }
        gloo_console::log!(format!(
            "traverse cost({})",
            instant::Instant::now().duration_since(start).as_millis()
        ));
        Self(result)
    }
}
