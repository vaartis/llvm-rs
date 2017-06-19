extern crate libc;

use std::convert::{From,TryFrom};

use value::*;
use value_kind::*;

#[link(name = "LLVM-4.0")]
extern "C" {
    fn LLVMValueAsBasicBlock(val: *const CValue) -> *const CBasicBlock;
    fn LLVMBasicBlockAsValue(val: *const CBasicBlock) -> *const CValue;
}

pub(super) enum CBasicBlock {}

#[derive(PartialEq,Eq)]
pub struct BasicBlock(pub(super) *const CBasicBlock);

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
