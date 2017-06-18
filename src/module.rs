extern crate libc;

use std::ffi::{CStr, CString};
use std::fmt;

use ::LLVMErr;

#[link(name = "LLVM-4.0")]
extern "C" {
    fn LLVMModuleCreateWithName(s: *const libc::c_char) -> *const CModule;
    fn LLVMPrintModuleToString(m: *const CModule) -> *const libc::c_char;
    fn LLVMDisposeModule(m: *const CModule);
}

enum CModule {}

pub struct Module {
    pub name: String,
    inner: *const CModule
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
