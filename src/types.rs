extern crate libc;

use std::slice;
use std::fmt;
use std::convert::{From,TryFrom};
use std::ffi::CStr;

use bindings::*;

pub use bindings::LLVMTypeKind as TypeKind;

#[derive(PartialEq,Eq,Copy,Clone)]
pub struct Type(pub(super) LLVMTypeRef);

impl Type {
    pub fn kind(&self) -> TypeKind {
        unsafe {
            LLVMGetTypeKind(self.0)
        }
    }

    pub fn is_sized(&self) -> bool {
        unsafe {
            LLVMTypeIsSized(self.0) == 1
        }
    }

    pub fn void() -> Type {
        Type(unsafe { LLVMVoidType() })
    }

    pub fn int1() -> Type { Type(unsafe { LLVMInt1Type() }) }
    pub fn int8() -> Type { Type(unsafe { LLVMInt8Type() }) }
    pub fn int16() -> Type { Type(unsafe { LLVMInt16Type() }) }
    pub fn int32() -> Type { Type(unsafe { LLVMInt32Type() }) }
    pub fn int64() -> Type { Type(unsafe { LLVMInt64Type() }) }
    pub fn int128() -> Type { Type(unsafe { LLVMInt128Type()}) }
    pub fn int(num: libc::c_uint) -> Type { Type(unsafe { LLVMIntType(num) }) } // c_uint is just an alias, so probably OK
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c_s = unsafe { CStr::from_ptr(LLVMPrintTypeToString(self.0)) };
        let st = c_s.to_str().unwrap(); // Propably valid utf8
        write!(f, "{}", st)
    }
}

impl From<FunctionType> for Type {
    fn from(t: FunctionType) -> Self {
        Type(t.0)
    }
}

impl TryFrom<Type> for FunctionType {
    type Error = ();
    // () - not a function type
    fn try_from(val: Type) -> Result<FunctionType, ()> {
        if val.kind() == TypeKind::LLVMFunctionTypeKind {
            Ok(FunctionType(val.0))
        } else {
            Err(())
        }
    }
}

#[derive(PartialEq,Eq)]
pub struct FunctionType(pub(super) LLVMTypeRef);

impl FunctionType {
    pub fn new(ret_type: Type, args: &[Type], is_vararg: bool) -> FunctionType {
        let args_ctypes = args.iter().map(|x| x.0).collect::<Vec<_>>().as_mut_ptr();
        FunctionType(unsafe { LLVMFunctionType(ret_type.0, args_ctypes, args.len() as libc::c_uint, is_vararg as i32) })
    }

    pub fn is_vararg(&self) -> bool {
        unsafe { LLVMIsFunctionVarArg(self.0) == 1 }
    }

    pub fn return_type(&self) -> Type {
        unsafe { Type(LLVMGetReturnType(self.0)) }
    }

    pub fn params(&self) -> Vec<Type> {
        unsafe {
            let ln = LLVMCountParamTypes(self.0) as usize;
            let arr = Vec::<LLVMTypeRef>::with_capacity(ln).as_mut_ptr();
            LLVMGetParamTypes(self.0, arr);
            let arr = arr as *mut LLVMTypeRef;
            slice::from_raw_parts(arr, ln)
                .iter().map(|x| Type(*x)).collect::<Vec<_>>()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Type,FunctionType,TypeKind};

    #[test]
    fn test_params() {
        let f = FunctionType::new(Type::void(), &vec![Type::int32(), Type::int8()], true);
        let pars = f.params();
        assert_eq!(pars.len(), 2);
        assert_eq!(pars[0], Type::int32());
        assert_eq!(pars[1], Type::int8());
    }

    #[test]
    fn test_return_type() {
        let f = FunctionType::new(Type::void(), &vec![], true);
        assert_eq!(f.return_type(), Type::void());
    }

    #[test]
    fn test_is_vararg() {
        let f = FunctionType::new(Type::int32(), &vec![], true);
        assert!(f.is_vararg());
        let f = FunctionType::new(Type::int32(), &vec![], false);
        assert!(!f.is_vararg());
    }

    #[test]
    fn test_type_kind() {
        assert_eq!(Type::int32().kind(), TypeKind::LLVMIntegerTypeKind);
        assert_eq!(Type::from(FunctionType::new(Type::int32(), &vec![], false)).kind(), TypeKind::LLVMFunctionTypeKind);
    }
}
