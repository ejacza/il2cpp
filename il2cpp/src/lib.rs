mod api;
mod dump;
mod tabledefs;

use core::ffi::c_void;
use std::sync::OnceLock;

static API: OnceLock<api::Il2CppApi> = OnceLock::new();

#[no_mangle]
pub extern "C" fn rust_il2cpp_api_init(handle: *mut c_void) {
    let a = api::init_api(handle);
    let _ = API.set(a);
}

#[no_mangle]
pub extern "C" fn rust_il2cpp_dump() {
    if let Some(a) = API.get() {
        dump::run_dump(a);
    }
}
