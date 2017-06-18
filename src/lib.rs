extern crate libc;

pub mod types;
pub mod module;
pub mod builder;
pub mod value;

#[derive(Debug)]
pub enum LLVMErr {
    NulByte
}
