use wasm_bindgen::prelude::*;
pub use renderer_core::*; // Re-export everything (FlexRenderer, FlexEngine etc)
use std::rc::Rc;
use std::cell::RefCell;

// Include generated assets
pub mod generated_assets {
    include!(concat!(env!("OUT_DIR"), "/generated_assets.rs"));
}

// Include generated UI
pub mod generated_ui;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"FlexEngine App Initialized".into());
    Ok(())
}

#[wasm_bindgen]
pub fn create_app_renderer(device: renderer_core::web_bindings::GpuDevice, context: renderer_core::web_bindings::GpuCanvasContext) -> FlexRenderer {
    let engine = Rc::new(RefCell::new(FlexEngine::new()));

    // Load embedded assets
    for key in generated_assets::ASSET_KEYS {
        if let Some(data) = generated_assets::get_asset(key) {
             let engine_key = format!("asset://{}", key);
             engine.borrow_mut().load_asset_bytes(&engine_key, data.to_vec());
        }
    }

    // Build the UI in a root scope
    let root_scope = renderer_core::signals::create_root(|s| {
        generated_ui::app::build(engine.clone(), None, generated_ui::app::Props {});
        s
    });
    engine.borrow_mut().root_scope_id = Some(root_scope.id);
    
    // Return the renderer, using the shared engine Rc
    FlexRenderer::new_with_ref(device, context, engine)
}

// Shim for load_image_to_engine to handle local assets
pub async fn load_image_to_engine(engine: Rc<RefCell<FlexEngine>>, url: String) {
    if url.starts_with("asset:") {
        let path = url.trim_start_matches("asset:");
        let clean_path = path.trim_start_matches('/');
        
        let bytes: Option<Vec<u8>> = generated_assets::get_asset(clean_path).map(|b| b.to_vec());
        
        if let Some(data) = bytes {
             // We need a way to pass this data to the engine.
             // FlexEngine likely has a method for this.
             // Looking at renderer-core, it had `load_image_to_engine` which handled this.
             // Since we removed that from renderer (or will), we need to implement it here using public API.
             
             // We need to check what public API FlexEngine exposes for loading images.
             // If usage was `engine.assets.insert(...)`, we need that to be public.
             
             engine.borrow_mut().load_asset_bytes(clean_path, data);
        }
    } else {
        // Fallback to renderer's web downloader
        let promise = renderer_core::web_bindings::download_image(&url);
        if let Ok(js_val) = wasm_bindgen_futures::JsFuture::from(promise).await {
             let uint8_array = js_sys::Uint8Array::new(&js_val);
             let bytes = uint8_array.to_vec();
             engine.borrow_mut().load_asset_bytes(&url, bytes);
        }
    }
}
