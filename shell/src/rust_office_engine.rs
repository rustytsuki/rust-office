use async_ffi::{LocalFfiFuture, FfiFuture, FutureExt};
use tokio::runtime::Runtime;

// #[link(name = "rust_office_engine")]
extern "C" {
    pub fn init_runtime() -> *mut Runtime;
    pub fn load_docx_dir(rt_ptr: *mut Runtime, dir: String) -> LocalFfiFuture<()>;
    pub fn api_test(rt_ptr: *mut Runtime) -> LocalFfiFuture<()>;
}
