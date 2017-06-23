extern crate libc;

use std::ops::Drop;

use basic_block::*;
use value::*;
use context::*;
use bindings::*;

pub struct IRBuilder(pub(super) LLVMBuilderRef);

impl IRBuilder {
    pub fn new() -> IRBuilder {
        IRBuilder(unsafe { LLVMCreateBuilder() })
    }

    pub fn new_in_context(cont: Context) -> IRBuilder {
        IRBuilder(unsafe { LLVMCreateBuilderInContext(cont.0) })
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
        let modl = Module::new("test");
        let f = modl.add_function("testf", FunctionType::new(Type::int32(), &vec![], false));
        let entry_b = f.append_bb("entry");
        let builder = IRBuilder::new();
        builder.position_at_end(f.entry_bb().unwrap());
        assert_eq!(builder.insertion_block(), entry_b);
    }
}
