extern crate libc;

use std::slice;
use std::fmt;
use std::convert::{From,TryFrom};
use std::ffi::CStr;

#[link(name = "LLVM-4.0")]
extern "C" {
    fn LLVMInt8Type() -> *const CType;
    fn LLVMInt16Type() -> *const CType;
    fn LLVMInt32Type() -> *const CType;
    fn LLVMInt64Type() -> *const CType;
    fn LLVMInt128Type() -> *const CType;
    fn LLVMIntType(num: libc::c_uint) -> *const CType;

    fn LLVMVoidType() -> *const CType;

    fn LLVMGetTypeKind(tp: *const CType) -> TypeKind;
    fn LLVMPrintTypeToString(tp: *const CType) -> *const libc::c_char;
    fn LLVMTypeIsSized(tp: *const CType) -> bool;

    fn LLVMTypeConstNull(tp: *const CType) -> *const CType;
    fn LLVMTypeConstPointerNull(tp: *const CType) -> *const CType;
    fn LLVMTypeGetUndef(tp: *const CType) -> *const CType;

    fn LLVMFunctionType(ret_type: *const CType , args: *const CType, param_count: libc::c_uint, is_vararg: bool) -> *const CType;
    fn LLVMIsFunctionVarArg(t: *const CType) -> bool;
    fn LLVMGetReturnType(t: *const CType) -> *const CType;
    fn LLVMCountParamTypes(t: *const CType) -> libc::c_uint;
    fn LLVMGetParamTypes(t: *const CType, ar: *const CType);
}

pub(super) enum CType {}

#[derive(PartialEq,Eq)]
pub struct Type(pub(super) *const CType);

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
pub enum TypeKind {
    /// type with no size
    Void,
    /// 16 bit floating point type
    Half,
    ///  32 bit floating point type
    Float,
    // 64 bit floating point type
    Double,
    /// 80 bit floating point type (X87)
    X86FP80,
    /// 128 bit floating point type (112-bit mantissa)
    FP128,
    /// 128 bit floating point type (two 64-bits)
    PPCFP128,
    // Labels
    Label,
    // Arbitrary bit width integers
    Integer,
    /// Functions
    Function,
    /// Structures
    Struct,
    /// Arrays
    Array,
    /// Pointers
    Pointer,
    /// SIMD 'packed' format, or other vector type
    Vector,
    /// Metadata
    Metadata,
    /// X86 MMX
    X86MMX,
    /// Tokens
    Token
}

impl Type {
    pub fn kind(&self) -> TypeKind {
        unsafe {
            LLVMGetTypeKind(self.0)
        }
    }

    pub fn sized(&self) -> bool {
        unsafe {
            LLVMTypeIsSized(self.0)
        }
    }

    pub fn int8() -> Type { Type(unsafe { LLVMInt8Type() }) }
    pub fn int16() -> Type { Type(unsafe { LLVMInt16Type() }) }
    pub fn int32() -> Type { Type(unsafe { LLVMInt32Type() }) }
    pub fn int64() -> Type { Type(unsafe { LLVMInt64Type() }) }
    pub fn int128() -> Type { Type(unsafe { LLVMInt128Type()}) }
    pub fn int(num: libc::c_uint) -> Type { Type(unsafe { LLVMIntType(num) }) } // c_uint is just an alias, so probably OK

    pub fn undef(tp: Type) -> Type {
        Type(unsafe { LLVMTypeGetUndef(tp.0) })
    }

    pub fn void() -> Type {
        Type(unsafe { LLVMVoidType() })
    }

    pub fn const_null(tp: Type) -> Type {
        Type(unsafe { LLVMTypeConstNull(tp.0) })
    }

    pub fn const_pointer_null(tp: Type) -> Type {
        Type(unsafe { LLVMTypeConstPointerNull(tp.0) })
    }
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
        if val.kind() == TypeKind::Function {
            Ok(FunctionType(val.0))
        } else {
            Err(())
        }
    }
}

#[derive(PartialEq,Eq)]
pub struct FunctionType(pub(super) *const CType);

impl FunctionType {
    pub fn new(ret_type: Type, args: &[Type], is_vararg: bool) -> FunctionType {
        let args_ctypes = args.iter().map(|x| x.0).collect::<Vec<_>>().as_ptr() as *const CType;
        FunctionType(unsafe { LLVMFunctionType(ret_type.0, args_ctypes, args.len() as libc::c_uint, is_vararg) })
    }

    pub fn is_vararg(&self) -> bool {
        unsafe { LLVMIsFunctionVarArg(self.0) }
    }

    pub fn return_type(&self) -> Type {
        unsafe { Type(LLVMGetReturnType(self.0)) }
    }

    pub fn params(&self) -> Vec<Type> {
        unsafe {
            let ln = LLVMCountParamTypes(self.0) as usize;
            let arr = Vec::<*const CType>::with_capacity(ln).as_mut_ptr() as *mut CType;
            LLVMGetParamTypes(self.0, arr);
            let arr = arr as *mut *const CType;
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
        assert!(pars.len() == 2);
        assert!(pars[0] == Type::int32());
        assert!(pars[1] == Type::int8());
    }

    #[test]
    fn test_return_type() {
        let f = FunctionType::new(Type::void(), &vec![], true);
        assert!(f.return_type() == Type::void());
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
        assert!(Type::int32().kind() == TypeKind::Integer);
        assert!(Type::from(FunctionType::new(Type::int32(), &vec![], false)).kind() == TypeKind::Function);
    }
}
