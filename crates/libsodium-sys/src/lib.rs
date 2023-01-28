#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
mod ffi {
  include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use ffi::*;

#[cfg(test)]
mod test {
  use super::ffi;
  use std::{
    process::{Command, Stdio},
    ptr,
  };

  struct Sodium;
  impl Sodium {
    fn new() -> Result<Self, ()> {
      if unsafe { ffi::sodium_init() } == 0 {
        Ok(Self)
      } else {
        Err(())
      }
    }

    /// See: [generic_hashing](https://libsodium.gitbook.io/doc/hashing/generic_hashing#usage)
    fn crypto_generichash(
      &self,
      output: &mut [u8],
      input: &[u8],
      key: Option<&[u8]>,
    ) -> Result<(), ()> {
      assert!(output.len() >= ffi::crypto_generichash_BYTES_MIN as usize);
      assert!(output.len() <= ffi::crypto_generichash_BYTES_MAX as usize);

      let (key, key_len) = if let Some(key) = key {
        assert!(key.len() >= ffi::crypto_generichash_KEYBYTES_MIN as usize);
        assert!(key.len() <= ffi::crypto_generichash_KEYBYTES_MAX as usize);

        (key.as_ptr(), key.len())
      } else {
        (ptr::null(), 0)
      };

      if unsafe {
        ffi::crypto_generichash(
          output.as_mut_ptr(),
          output.len(),
          input.as_ptr(),
          input.len() as u64,
          key,
          key_len,
        )
      } < 0
      {
        Err(())
      } else {
        Ok(())
      }
    }
  }

  #[test]
  fn test() {
    let s = Sodium::new().unwrap();
    const OUT_LEN: usize = 32;
    let output = &mut [0u8; OUT_LEN];
    s.crypto_generichash(output, b"hello world", None).unwrap();
    let result = hex::encode(output);

    let res = Command::new("echo")
      .arg("-n")
      .arg("hello world")
      .stdout(Stdio::piped())
      .spawn()
      .unwrap();

    let arg = 32 * 8;

    let res = Command::new("b2sum")
      .arg("-l".to_owned() + &arg.to_string())
      .stdin(Stdio::from(res.stdout.unwrap()))
      .output()
      .unwrap();

    dbg!(result, String::from_utf8_lossy(&res.stdout));
  }
}
