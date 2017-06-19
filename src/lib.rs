#![feature(try_from, concat_idents)]

extern crate libc;

pub mod types;
pub mod module;
pub mod builder;
pub mod value;
pub mod value_kind;
pub mod function;
pub mod basic_block;

#[derive(Debug)]
pub enum LLVMErr {
    NulByte
}
