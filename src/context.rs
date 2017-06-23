use std::ops::Drop;

pub(super) enum CContext {}

pub struct Context(pub(super) *const CContext);

extern "C" {
    fn LLVMContextCreate() -> *const CContext;
    fn LLVMGetGlobalContext() -> *const CContext;
    fn LLVMContextDispose(cont: *const CContext);
}

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
