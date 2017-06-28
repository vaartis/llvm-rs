extern crate libc;

use std::ffi::{CString,CStr};
use std::convert::{From,TryFrom};
use std::fmt;

use value::{Value,ValueKind};
use basic_block::BasicBlock;
use bindings::*;

#[derive(PartialEq,Eq,Copy,Clone)]
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

impl TryFrom<Value> for Function {
    type Error = ();
    fn try_from(other: Value) -> Result<Function, ()> {
        if other.kind() == ValueKind::LLVMFunctionValueKind {
            Ok(Function(other.0))
        } else {
            Err(())
        }
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c_s = unsafe { CStr::from_ptr(LLVMPrintValueToString(self.0)) };
        let st = c_s.to_str().unwrap(); // Propably valid utf8
        write!(f, "{}", st)
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
