extern crate libc;

use std::ops::Drop;
use std::collections::HashMap;
use std::ffi::CString;

use basic_block::BasicBlock;
use value::Value;
use context::Context;
use switch::Switch;
use function::Function;
use function_call::FunctionCall;
use bindings::*;

#[derive(PartialEq,Eq)]
pub struct IRBuilder(pub(super) LLVMBuilderRef);

impl IRBuilder {
    pub fn new() -> IRBuilder {
        IRBuilder(unsafe { LLVMCreateBuilder() })
    }

    pub fn new_in_context(cont: &Context) -> IRBuilder {
        IRBuilder(unsafe { LLVMCreateBuilderInContext(cont.0) })
    }

    pub fn position_at_end(&self, bb: BasicBlock) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.0, bb.0);
        }
    }

    pub fn insertion_block(&self) -> Option<BasicBlock> {
        let r = unsafe { LLVMGetInsertBlock(self.0) };
        if r.is_null() { None } else { Some(BasicBlock(r)) }
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

    pub fn cond_br(&self, cond: Value, then: &BasicBlock, els: &BasicBlock) -> Value {
        Value(unsafe { LLVMBuildCondBr(self.0, cond.0, then.0, els.0) })
    }

    pub fn switch(&self, val: Value, default: BasicBlock, cases: HashMap<Value, BasicBlock>) -> Switch {
        let switch = unsafe {
            LLVMBuildSwitch(self.0, val.0, default.0, cases.len() as u32)
        };
        for (on_val, dest) in cases {
            unsafe {
                LLVMAddCase(switch, on_val.0, dest.0);
            }
        }
        Switch(switch)
    }

    pub fn call(&self, f: Function, args: &[Value]) -> FunctionCall {
        unsafe {
            FunctionCall(LLVMBuildCall(self.0,
                                       f.0,
                                       args.iter().map(|x| x.0).collect::<Vec<_>>().as_mut_ptr(),
                                       args.len() as u32,
                                       CString::new("").unwrap().as_ptr()
            )
            )
        }
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
    use module::Module;
    use types::{Type,FunctionType};
    use value::Value;

    #[test]
    fn test_insertion_block_and_position_at_end() {
        let modl = Module::new("test");
        let f = modl.add_function("testf", FunctionType::new(Type::int32(), &vec![], false));
        let entry_b = f.append_bb("entry");
        let builder = IRBuilder::new();
        builder.position_at_end(f.entry_bb().unwrap());
        assert_eq!(builder.insertion_block().unwrap(), entry_b);
    }

    #[test]
    fn test_function_calling() {
        let modl = Module::new("test");
        let f = modl.add_function("testf", FunctionType::new(Type::int32(), &vec![], false));
        let f2 = modl.add_function("testf2", FunctionType::new(Type::int32(), &vec![], false));
        let _ = f.append_bb("entry");
        let _ = f2.append_bb("entry");
        let builder = IRBuilder::new();
        builder.position_at_end(f.entry_bb().unwrap());
        let call = builder.call(f2, &vec![Value::const_int(Type::int32(), 10)]);
        assert_eq!(call.called_value(), f2);
    }
}
