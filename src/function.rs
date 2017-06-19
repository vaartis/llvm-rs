extern crate libc;

use ::types::*;
use ::value::*;

pub struct Function(pub(super) *const CValue);
