use std::{ffi::CString, ptr};

use napi_sys::*;

#[no_mangle]
pub unsafe extern "C" fn napi_register_module_v1(
  env: *const napi_env,
  exports: *mut napi_value,
) -> *mut napi_value {
  let key = CString::new("foo").unwrap();
  let value = CString::new("bar").unwrap();

  let mut str = ptr::null_mut();
  assert!(napi_create_string_utf8(env, value.as_ptr(), 3, &mut str) == 0);
  assert!(napi_set_named_property(env, exports, key.as_ptr(), str) == 0);

  exports
}
