extern crate libc;

use std::ffi::CStr;
use std::fmt;

//use ::types::*;

pub(super) enum CValue {}
pub struct Value {
    pub(super) inner: *const CValue
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c_s = unsafe { CStr::from_ptr(LLVMPrintValueToString(self.inner)) };
        let st = c_s.to_str().unwrap(); // Propably valid utf8
        write!(f, "{}", st)
    }
}

#[link(name = "LLVM-4.0")]
extern "C" {
    fn LLVMPrintValueToString(v: *const CValue) -> *const libc::c_char;
}
