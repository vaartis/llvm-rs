extern crate libc;

use std::ffi::CString;
use std::convert::From;

use value::*;
use basic_block::*;
use bindings::*;

pub struct Function(pub(super) LLVMValueRef);

impl Function {
    pub fn entry_bb(&self) -> Option<BasicBlock> {
        let bb = unsafe {
            LLVMGetEntryBasicBlock(self.0)
        };
        if bb.is_null() {
            None
        } else {
            Some(BasicBlock(bb))
        }
    }

    pub fn append_bb(&self, name: &str) -> BasicBlock {
        let c_name = CString::new(name).unwrap();
        unsafe {
            BasicBlock(LLVMAppendBasicBlock(self.0, c_name.as_ptr()))
        }
    }
}

impl From<Function> for Value {
    fn from(other: Function) -> Value {
        Value(other.0)
    }
}

#[cfg(test)]
mod tests {
    use module::Module;
    use types::{Type, FunctionType};

    #[test]
    fn test_entry_bb() {
        let modl = Module::new("test");
        let f = modl.add_function("testf", FunctionType::new(Type::int32(), &vec![], false));
        let bl = f.append_bb("entry");
        assert_eq!(f.entry_bb().unwrap(), bl);
    }
}
