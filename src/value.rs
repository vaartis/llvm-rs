extern crate libc;

use std::ffi::CStr;
use std::fmt;

use ::types::*;

extern "C" {
    pub(super) fn LLVMPrintValueToString(v: *const CValue) -> *const libc::c_char;
    fn LLVMTypeOf(v: *const CValue) -> *const CType;

    fn LLVMConstInt(tp: *const CType, num: libc::c_ulonglong, sig_ext: bool) -> *const CValue;
}

pub(super) enum CValue {}
pub struct Value(pub(super) *const CValue);

impl Value {
    pub fn type_of(&self) -> Type {
        unsafe { Type(LLVMTypeOf(self.0)) }
    }

    pub fn const_int(tp: Type, num: libc::c_ulonglong, sign_extended: bool) -> Value {
        unsafe { Value(LLVMConstInt(tp.0, num, sign_extended)) }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c_s = unsafe { CStr::from_ptr(LLVMPrintValueToString(self.0)) };
        let st = c_s.to_str().unwrap(); // Propably valid utf8
        write!(f, "{}", st)
    }
}
