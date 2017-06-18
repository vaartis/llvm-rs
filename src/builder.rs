extern crate libc;

#[link(name = "LLVM-4.0")]
extern "C" {
    fn LLVMCreateBuilder() -> *const CBuilder;
}

pub(super) enum CBuilder {}

pub struct IRBuilder {
    pub(super) inner: *const CBuilder
}

impl IRBuilder {
    fn new() -> IRBuilder {
        IRBuilder{inner: unsafe { LLVMCreateBuilder() }}
    }
}
