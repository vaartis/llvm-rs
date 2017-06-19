#![feature(try_from)]

extern crate libc;

pub mod types;
pub mod module;
pub mod builder;
pub mod value;
pub mod function;

#[derive(Debug)]
pub enum LLVMErr {
    NulByte
}
