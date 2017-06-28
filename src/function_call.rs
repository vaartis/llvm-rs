use std::ffi::CStr;
use std::fmt;

use function::Function;
use bindings::*;

#[derive(PartialEq,Eq,Copy,Clone)]
pub struct FunctionCall(pub(super) LLVMValueRef);

impl FunctionCall {
    pub fn set_tail_call(&self, isa: bool) {
        unsafe {
            LLVMSetTailCall(self.0, isa as i32);
        }
    }

    pub fn is_tail_call(&self) -> bool {
        unsafe {
            LLVMIsTailCall(self.0) == 1
        }
    }

    pub fn called_value(&self) -> Function {
        Function(unsafe { LLVMGetCalledValue(self.0) })
    }
}

impl fmt::Debug for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c_s = unsafe { CStr::from_ptr(LLVMPrintValueToString(self.0)) };
        let st = c_s.to_str().unwrap(); // Propably valid utf8
        write!(f, "{}", st)
    }
}
