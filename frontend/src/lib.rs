use app::App;
use wasm_bindgen::prelude::wasm_bindgen;

// Entrypoint for WASM
#[wasm_bindgen]
pub fn hydrate() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
