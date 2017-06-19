extern crate libc;

use std::ffi::{CStr, CString};
use std::fmt;

use ::types::*;
use ::value::*;
use ::function::Function;

#[link(name = "LLVM-4.0")]
extern "C" {
    fn LLVMModuleCreateWithName(s: *const libc::c_char) -> *const CModule;
    fn LLVMPrintModuleToString(m: *const CModule) -> *const libc::c_char;
    fn LLVMDisposeModule(m: *const CModule);

    fn LLVMAddFunction(m: *const CModule, nm: *const libc::c_char, tp: *const CType) -> *const CValue;
    fn LLVMGetNamedFunction(m: *const CModule, nm: *const libc::c_char) -> *const CValue;
    fn LLVMGetFirstFunction(m: *const CModule) -> *const CValue;
    fn LLVMGetNextFunction(m: *const CValue) -> *const CValue;
}

pub(super) enum CModule {}

pub struct Module(pub(super) *const CModule);

impl Module {
    pub fn new_with_name(name: &str) -> Self {
        let c_name = CString::new(name).unwrap();

        let c_modl = unsafe { LLVMModuleCreateWithName(c_name.as_ptr()) };
        Module(c_modl)
    }

    pub fn add_function(&self, name: &str, tp: FunctionType) -> Function {
        let c_name = CString::new(name).unwrap();
        Function(unsafe { LLVMAddFunction(self.0, c_name.as_ptr(), tp.0) })
    }

    pub fn find_function(&self, name: &str) -> Option<Value> {
        let c_name = CString::new(name).unwrap();

        let f = unsafe { LLVMGetNamedFunction(self.0, c_name.as_ptr()) };

        if f.is_null() { None } else { Some(Value(f)) }
    }

    pub fn functions(&self) -> Vec<Function> {
        let f = unsafe { LLVMGetFirstFunction(self.0) };
        if f.is_null() {
            vec![]
        } else {
            let mut res = vec![Function(f)];
            let mut current = f;
            loop {
                let next = unsafe { LLVMGetNextFunction(current) };
                if !next.is_null() {
                    res.push(Function(next));
                    current = next;
                } else {
                    break;
                }
            }
            res
        }
    }
}

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c_s = unsafe { CStr::from_ptr(LLVMPrintModuleToString(self.0)) };
        let st = c_s.to_str().unwrap(); // Propably valid utf8
        write!(f, "{}", st)
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe { LLVMDisposeModule(self.0) }
    }
}

#[cfg(test)]
mod tests {
    use super::Module;
    use ::types::{Type, FunctionType};

    #[test]
    fn test_add_function() {
        let modl = Module::new_with_name("test");
        let _ = modl.add_function("testf", FunctionType::new(Type::int32(), &vec![], false));
        assert!(modl.find_function("testf").is_some());
        let _ = modl.add_function("testf2", FunctionType::new(Type::int32(), &vec![], false));

        assert!(modl.functions().len() == 2);
    }
}
