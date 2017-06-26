#![feature(try_from, concat_idents)]

extern crate libc;

mod bindings;

pub mod types;
pub mod module;
pub mod builder;
pub mod value;
pub mod function;
pub mod basic_block;
pub mod context;
pub mod switch;

#[derive(Debug)]
pub enum LLVMErr {
    NulByte
}
