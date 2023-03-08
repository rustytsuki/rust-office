#[cfg(target_arch = "wasm32")]
use js_sys::Promise;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::future_to_promise;

pub mod app;

#[cfg(not(target_arch = "wasm32"))]
mod rust_office_engine;

#[cfg(target_arch = "wasm32")]
use rust_office_engine;

#[cfg(not(target_arch = "wasm32"))]
pub fn run_native() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            rust_office().await;
        })
}

#[cfg(not(target_arch = "wasm32"))]
async fn rust_office() {
    unsafe {
        let rt = rust_office_engine::init_runtime();
        // rust_office_engine::load_docx_dir(rt, "D:/docx".to_string()).await;
        rust_office_engine::api_test(rt).await;
    }

    let mut native_options = eframe::NativeOptions::default();
    // native_options.maximized = true;
    // native_options.centered = true;
    native_options.initial_window_size = Some(eframe::egui::vec2(800., 600.));

    #[cfg(feature = "glow")] {
        native_options.renderer = eframe::Renderer::Glow;
    }

    let result = eframe::run_native(
        "eframe template",
        native_options.clone(),
        Box::new(|cc| Box::new(app::RustOfficeApp::new(cc))),
    );

    if cfg!(feature = "glow") && result.is_err() {
        // fallback skia
        #[cfg(feature = "skia")] {
            native_options.renderer = eframe::Renderer::Skia;
            eframe::run_native(
                "eframe template",
                native_options,
                Box::new(|cc| Box::new(app::RustOfficeApp::new(cc))),
            )
            .unwrap();
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct RustOffice {}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl RustOffice {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<RustOffice, JsValue> {
        Ok(RustOffice {})
    }

    pub fn run(&self) {
        let web_options = eframe::WebOptions::default();

        wasm_bindgen_futures::spawn_local(async {
            eframe::start_web("rust-office", web_options, Box::new(|cc| Box::new(app::RustOfficeApp::new(cc))))
                .await
                .expect("failed to start eframe");
        });
    }

    // https://github.com/rustwasm/wasm-bindgen/issues/2195
    pub fn load(&self, uri: String, tree: String) -> Promise {
        rust_office_engine::log::info(&tree);
        future_to_promise(async move {
            let ret = rust_office_engine::xml::web::load_docx_uri(uri, tree).await;

            Ok(ret.into())
        })
    }
}

#[cfg(target_arch = "wasm32")]
impl Drop for RustOffice {
    fn drop(&mut self) {}
}
