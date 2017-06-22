extern crate libc;

use std::ops::Drop;
use ::basic_block::*;
use ::value::*;

extern "C" {
    fn LLVMCreateBuilder() -> *const CBuilder;
    fn LLVMPositionBuilderAtEnd(b: *const CBuilder, bb: *const CBasicBlock);
    fn LLVMGetInsertBlock(bld: *const CBuilder) -> *const CBasicBlock;
    fn LLVMDisposeBuilder(bld: *const CBuilder);

    fn LLVMBuildRetVoid(bld: *const CBuilder) -> *const CValue;
    fn LLVMBuildRet(bld: *const CBuilder, val: *const CValue) -> *const CValue;
    fn LLVMBuildBr(bld: *const CBuilder, bb: *const CBasicBlock) -> *const CValue;
}

pub(super) enum CBuilder {}

pub struct IRBuilder(pub(super) *const CBuilder);

impl IRBuilder {
    pub fn new() -> IRBuilder {
        IRBuilder(unsafe { LLVMCreateBuilder() })
    }

    pub fn position_at_end(&self, bb: BasicBlock) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.0, bb.0);
        }
    }

    pub fn insertion_block(&self) -> BasicBlock {
        BasicBlock(unsafe { LLVMGetInsertBlock(self.0) })
    }

    pub fn ret_void(&self) -> Value {
        Value(unsafe { LLVMBuildRetVoid(self.0) })
    }

    pub fn ret(&self, val: Value) -> Value {
        Value(unsafe { LLVMBuildRet(self.0, val.0) })
    }

    pub fn br(&self, br: BasicBlock) -> Value {
        Value(unsafe { LLVMBuildBr(self.0, br.0) })
    }
}

impl Drop for IRBuilder {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IRBuilder;
    use ::module::*;
    use ::types::*;

    #[test]
    fn test_insertion_block_and_position_at_end() {
        let modl = Module::new_with_name("test");
        let f = modl.add_function("testf", FunctionType::new(Type::int32(), &vec![], false));
        let entry_b = f.append_bb("entry");
        let builder = IRBuilder::new();
        builder.position_at_end(f.entry_bb().unwrap());
        assert_eq!(builder.insertion_block(), entry_b);
    }
}
