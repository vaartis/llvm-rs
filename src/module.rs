extern crate libc;

use std::ffi::{CStr, CString};
use std::fmt;

use context::*;
use types::*;
use value::*;
use function::*;
use bindings::*;

pub struct Module(pub(super) LLVMModuleRef);

impl Module {
    pub fn new(name: &str) -> Module {
        let c_name = CString::new(name).unwrap();

        Module(unsafe { LLVMModuleCreateWithName(c_name.as_ptr()) })
    }

    pub fn new_in_context(name: &str, cont: Context) -> Module {
        let c_name = CString::new(name).unwrap();
        Module(unsafe { LLVMModuleCreateWithNameInContext(c_name.as_ptr(), cont.0) })
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
        let modl = Module::new("test");
        let _ = modl.add_function("testf", FunctionType::new(Type::int32(), &vec![], false));
        assert!(modl.find_function("testf").is_some());
        let _ = modl.add_function("testf2", FunctionType::new(Type::int32(), &vec![], false));

        assert_eq!(modl.functions().len(), 2);
    }
}
