extern crate libc;

use std::ffi::CStr;
use std::fmt;

use ::types::*;

#[link(name = "LLVM-4.0")]
extern "C" {
    fn LLVMPrintValueToString(v: *const CValue) -> *const libc::c_char;
    fn LLVMTypeOf(v: *const CValue) -> *const CType;
}

pub(super) enum CValue {}
pub struct Value(pub(super) *const CValue);

impl Value {
    pub fn type_of(&self) -> Type {
        unsafe { Type(LLVMTypeOf(self.0)) }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c_s = unsafe { CStr::from_ptr(LLVMPrintValueToString(self.0)) };
        let st = c_s.to_str().unwrap(); // Propably valid utf8
        write!(f, "{}", st)
    }
}
