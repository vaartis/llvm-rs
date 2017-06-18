extern crate libc;

use libc::c_char;

use std::ffi::{CString, CStr};
use std::fmt;

enum CModule {}

pub struct Module {
    pub name: String,
    inner: *mut CModule
}

#[derive(Debug)]
pub enum LLVMErr {
    NulByte
}

impl Module {
    pub fn new_with_name(name: &str) -> Result<Self, LLVMErr> {
        let c_name = match CString::new(name) {
            Ok(r) => r,
            Err(_) => return Err(LLVMErr::NulByte)
        };

        let c_modl = unsafe { LLVMModuleCreateWithName(c_name.as_ptr()) };
        Ok(Module{name: name.to_string(), inner: c_modl})
    }
}

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c_s = unsafe { CStr::from_ptr(LLVMPrintModuleToString(self.inner)) };
        let st = c_s.to_str().unwrap(); // Propably valid utf8
        write!(f, "{}", st)
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe { LLVMDisposeModule(self.inner) }
    }
}

#[link(name = "LLVM-4.0")]
extern "C" {
    fn LLVMModuleCreateWithName(s: *const c_char) -> *mut CModule;
    fn LLVMPrintModuleToString(m: *const CModule) -> *const c_char;
    fn LLVMDisposeModule(m: *const CModule);
}

#[test]
fn create_module() {
    let module = Module::new_with_name("test").unwrap();
    println!("{:?}", module);
}
