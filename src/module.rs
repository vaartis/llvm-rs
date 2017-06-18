extern crate libc;

use std::ffi::{CStr, CString};
use std::fmt;

use ::LLVMErr;

use ::types::*;
use ::value::*;

#[link(name = "LLVM-4.0")]
extern "C" {
    fn LLVMModuleCreateWithName(s: *const libc::c_char) -> *const CModule;
    fn LLVMPrintModuleToString(m: *const CModule) -> *const libc::c_char;
    fn LLVMDisposeModule(m: *const CModule);

    fn LLVMAddFunction(m: *const CModule, nm: *const libc::c_char, tp: *const CType) -> *const CValue;
    fn LLVMGetNamedFunction(m: *const CModule, nm: *const libc::c_char) -> *const CValue;
}

pub(super) enum CModule {}

pub struct Module {
    pub name: String,
    pub(super) inner: *const CModule
}

impl Module {
    pub fn new_with_name(name: &str) -> Self {
        let c_name = CString::new(name).unwrap();

        let c_modl = unsafe { LLVMModuleCreateWithName(c_name.as_ptr()) };
        Module{name: name.to_string(), inner: c_modl}
    }

    pub fn add_function(&self, name: &str, tp: Type) -> Value {
        debug_assert!(tp.kind() == TypeKind::Function);

        let c_name = CString::new(name).unwrap();
        Value{inner: unsafe { LLVMAddFunction(self.inner, c_name.as_ptr(), tp.inner) } }
    }

    pub fn find_function(&self, name: &str) -> Option<Value> {
        let c_name = CString::new(name).unwrap();

        let f = unsafe { LLVMGetNamedFunction(self.inner, c_name.as_ptr()) };

        if f.is_null() { None } else { Some(Value{inner: f }) }
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

#[test]
fn test_add_function() {
    let modl = Module::new_with_name("test");
    let f = modl.add_function("testf", Type::function_type(Type::int32(), &vec![], false));
    assert!(modl.find_function("testf").is_some());
}
