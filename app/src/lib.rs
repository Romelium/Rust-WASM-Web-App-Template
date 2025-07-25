// app/src/lib.rs

// Module declarations
pub mod app_handle;
mod dom_creation;
mod drawing_app;
mod event_listeners;
pub mod mount;
mod rendering;
mod utils;

// Re-export key types for wasm-bindgen
pub use app_handle::AppHandle;
pub use mount::mount_app;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main_js_startup() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    web_sys::console::log_1(&"WASM module loaded.".into());
    Ok(())
}
