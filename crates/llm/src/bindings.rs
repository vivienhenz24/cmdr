//! FFI bindings for llama.cpp
//! 
//! This module contains the raw FFI bindings to the llama.cpp C API.

use std::ffi::{c_char, c_int, c_void};

#[cfg(feature = "native-llama")]
mod native_bindings {
    use super::*;
    
    #[link(name = "llama")]
    extern "C" {
        // TODO: Add actual llama.cpp function bindings
        // For now, these are placeholder bindings
        
        pub fn llama_backend_init(numa: bool) -> c_int;
        pub fn llama_backend_free();
        
        // Model loading
        pub fn llama_load_model_from_file(
            path_model: *const c_char,
            params: *const c_void,
        ) -> *mut c_void;
        
        // Context creation
        pub fn llama_new_context_with_model(
            model: *const c_void,
            params: *const c_void,
        ) -> *mut c_void;
        
        // Generation
        pub fn llama_eval(
            ctx: *mut c_void,
            tokens: *const i32,
            n_tokens: c_int,
            n_past: c_int,
            n_threads: c_int,
        ) -> c_int;
    }
}

#[cfg(feature = "native-llama")]
pub use native_bindings::*;

#[cfg(not(feature = "native-llama"))]
pub mod dummy {
    use super::*;
    #[allow(dead_code, unused_variables)]
    pub unsafe fn llama_backend_init(_numa: bool) -> c_int { 0 }
    #[allow(dead_code)]
    pub unsafe fn llama_backend_free() {}
    #[allow(dead_code, unused_variables)]
    pub unsafe fn llama_load_model_from_file(_path_model: *const c_char, _params: *const c_void) -> *mut c_void { std::ptr::null_mut() }
    #[allow(dead_code, unused_variables)]
    pub unsafe fn llama_new_context_with_model(_model: *const c_void, _params: *const c_void) -> *mut c_void { std::ptr::null_mut() }
    #[allow(dead_code, unused_variables)]
    pub unsafe fn llama_eval(_ctx: *mut c_void, _tokens: *const i32, _n_tokens: c_int, _n_past: c_int, _n_threads: c_int) -> c_int { 0 }
} 