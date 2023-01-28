mod ffi {
  use std::marker::PhantomData;

  #[repr(C)]
  pub struct napi_env {
    _data: [u8; 0],
    _marker: PhantomData<*mut ()>,
  }

  #[repr(C)]
  pub struct napi_value {
    _data: [u8; 0],
    _marker: PhantomData<*mut ()>,
  }

  extern "C" {
    pub fn napi_set_named_property(
      env: *const napi_env,
      object: *const napi_value,
      utf8Name: *const std::os::raw::c_char,
      value: *const napi_value,
    ) -> u8;
    pub fn napi_create_string_utf8(
      env: *const napi_env,
      str: *const std::os::raw::c_char,
      length: usize,
      result: *mut *mut napi_value,
    ) -> u8;
  }
}

pub use ffi::*;
