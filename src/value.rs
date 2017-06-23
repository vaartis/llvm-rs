extern crate libc;

use std::ffi::CStr;
use std::fmt;

use types::*;
use bindings::*;

pub struct Value(pub(super) LLVMValueRef);

impl Value {
    pub fn type_of(&self) -> Type {
        unsafe { Type(LLVMTypeOf(self.0)) }
    }

    pub fn const_int(tp: Type, num: libc::c_ulonglong, sign_extended: bool) -> Value {
        unsafe { Value(LLVMConstInt(tp.0, num, sign_extended as i32)) }
    }

    pub fn undef(tp: Type) -> Value {
        Value(unsafe { LLVMGetUndef(tp.0) })
    }

    pub fn const_null(tp: Type) -> Value {
        Value(unsafe { LLVMConstNull(tp.0) })
    }

    pub fn const_pointer_null(tp: Type) -> Value {
        Value(unsafe { LLVMConstPointerNull(tp.0) })
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c_s = unsafe { CStr::from_ptr(LLVMPrintValueToString(self.0)) };
        let st = c_s.to_str().unwrap(); // Propably valid utf8
        write!(f, "{}", st)
    }
}
