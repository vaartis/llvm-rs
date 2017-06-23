extern crate libc;

use std::convert::{From,TryFrom};
use std::fmt;
use std::ffi::CStr;

use value::*;
use value_kind::*;
use bindings::*;

#[derive(PartialEq,Eq)]
pub struct BasicBlock(pub(super) LLVMBasicBlockRef);

impl From<BasicBlock> for Value {
    fn from(other: BasicBlock) -> Value {
        unsafe {
            Value(LLVMBasicBlockAsValue(other.0))
        }
    }
}

impl TryFrom<Value> for BasicBlock {
    type Error = ();
    fn try_from(other: Value) -> Result<BasicBlock, ()> {
        if other.classify() == ValueKind::BasicBlock {
            unsafe {
                Ok(BasicBlock(LLVMValueAsBasicBlock(other.0)))
            }
        } else {
            Err(())
        }
    }
}

impl fmt::Debug for BasicBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c_s = unsafe { CStr::from_ptr(LLVMPrintValueToString(LLVMBasicBlockAsValue(self.0))) };
        let st = c_s.to_str().unwrap(); // Propably valid utf8
        write!(f, "{}", st)
    }
}
