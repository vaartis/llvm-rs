extern crate libc;

use std::ffi::CStr;
use std::fmt;

use types::Type;
use bindings::*;

pub use bindings::{LLVMValueKind as ValueKind, LLVMOpcode as Opcode};

#[derive(PartialEq,Eq,Copy,Clone,Hash)]
pub struct Value(pub(super) LLVMValueRef);

impl Value {
    pub fn kind(&self) -> ValueKind {
        unsafe {
            LLVMGetValueKind(self.0)
        }
    }

    pub fn opcode(&self) -> Opcode {
        unsafe { LLVMGetInstructionOpcode(self.0) }
    }

    pub fn type_of(&self) -> Type {
        unsafe { Type(LLVMTypeOf(self.0)) }
    }

    pub fn const_int(tp: Type, num: libc::c_ulonglong) -> Value {
        unsafe { Value(LLVMConstInt(tp.0, num, false as i32)) }
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
