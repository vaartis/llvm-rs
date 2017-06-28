use std::ops::Drop;

use bindings::*;

#[derive(PartialEq,Eq)]
pub struct Context(pub(super) LLVMContextRef);

impl Context {
    pub fn new() -> Context {
        Context(unsafe { LLVMContextCreate() })
    }

    pub fn global() -> Context {
        Context(unsafe { LLVMGetGlobalContext() })
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { LLVMContextDispose(self.0); }
    }
}
