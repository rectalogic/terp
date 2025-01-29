use crate::{app::AppPlugin, cli::Args, project::LoadProject};
use wasm_bindgen::prelude::*;

// wasm-pack build --target web

#[wasm_bindgen]
pub fn play(data: Vec<u8>) {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    AppPlugin::Player(Args::new(None)).run_with_event(LoadProject::new(data));
}
