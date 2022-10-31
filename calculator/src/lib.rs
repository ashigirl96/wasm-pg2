mod utils;

use std::fmt::Debug;
use rand::distributions::Distribution;
use rand::Rng;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

fn bare_bones() {
    log_u32(42);
    log_many("Logging", "many values!");
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}


#[wasm_bindgen]
extern {
    fn alert(s: &str);
    fn console_log(s: &str);
}

#[wasm_bindgen]
pub fn greet() -> String {
    "WebAssembly".to_string()
}

#[wasm_bindgen]
pub fn calculate_pi() -> f64 {
    let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します
    let dist = rand::distributions::Uniform::new(0.0, 1.0);

    let n = 1_000_000;
    let mut m = 0;
    for _ in 0..n {
        let x = dist.sample(&mut rng);
        let y = dist.sample(&mut rng);
        let dist = hypot(x, y);
        console_log!("dist = {}", dist);
        if dist <= 1.0 {
            m += 1;
        }
    }
    4.0 * m as f64 / n as f64
}

#[no_mangle]
fn hypot(x:f64, y:f64)-> f64 {
    let num = x.powi(2) + y.powi(2);
    num.powf(0.5)
}