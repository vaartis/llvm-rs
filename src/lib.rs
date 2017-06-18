extern crate libc;

pub mod types;
pub mod module;

#[derive(Debug)]
pub enum LLVMErr {
    NulByte
}
