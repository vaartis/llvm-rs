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
pub struct Type {
    pub(super) inner: *const CType
}

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
            LLVMGetTypeKind(self.inner)
        }
    }

    pub fn sized(&self) -> bool {
        unsafe {
            LLVMTypeIsSized(self.inner)
        }
    }

    pub fn int8() -> Type { Type{inner: unsafe { LLVMInt8Type() }} }
    pub fn int16() -> Type { Type{inner: unsafe { LLVMInt16Type() }} }
    pub fn int32() -> Type { Type{inner: unsafe { LLVMInt32Type() }} }
    pub fn int64() -> Type { Type{inner: unsafe { LLVMInt64Type() }} }
    pub fn int128() -> Type { Type{inner: unsafe { LLVMInt128Type() }} }
    pub fn int(num: libc::c_uint) -> Type { Type{inner: unsafe { LLVMIntType(num) }} } // c_uint is just an alias, so probably OK

    pub fn undef(tp: Type) -> Type {
        Type{inner: unsafe { LLVMTypeGetUndef(tp.inner) } }
    }

    pub fn void() -> Type {
        Type{inner: unsafe { LLVMVoidType() } }
    }

    pub fn const_null(tp: Type) -> Type {
        Type{inner: unsafe { LLVMTypeConstNull(tp.inner) } }
    }

    pub fn const_pointer_null(tp: Type) -> Type {
        Type{inner: unsafe { LLVMTypeConstPointerNull(tp.inner) } }
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c_s = unsafe { CStr::from_ptr(LLVMPrintTypeToString(self.inner)) };
        let st = c_s.to_str().unwrap(); // Propably valid utf8
        write!(f, "{}", st)
    }
}

impl From<FunctionType> for Type {
    fn from(t: FunctionType) -> Self {
        Type{inner: t.inner}
    }
}

impl TryFrom<Type> for FunctionType {
    type Error = ();
    // () - not a function type
    fn try_from(val: Type) -> Result<FunctionType, ()> {
        if val.kind() == TypeKind::Function {
            Ok(FunctionType{inner: val.inner})
        } else {
            Err(())
        }
    }
}

#[derive(PartialEq,Eq)]
pub struct FunctionType {
    pub(super) inner: *const CType
}

impl FunctionType {
    pub fn new(ret_type: Type, args: &[Type], is_vararg: bool) -> FunctionType {
        let c_ret_t = ret_type.inner;
        let args_ctypes = args.iter().map(|x| x.inner).collect::<Vec<_>>().as_ptr() as *const CType;
        FunctionType{inner: unsafe { LLVMFunctionType(c_ret_t, args_ctypes, args.len() as libc::c_uint, is_vararg) }}
    }

    pub fn is_vararg(&self) -> bool {
        unsafe { LLVMIsFunctionVarArg(self.inner) }
    }

    pub fn return_type(&self) -> Type {
        unsafe { Type{inner: LLVMGetReturnType(self.inner)} }
    }

    pub fn params(&self) -> Vec<Type> {
        unsafe {
            let ln = LLVMCountParamTypes(self.inner) as usize;
            let arr = Vec::<*const CType>::with_capacity(ln).as_mut_ptr() as *mut CType;
            LLVMGetParamTypes(self.inner, arr);
            let arr = arr as *mut *const CType;
            slice::from_raw_parts(arr, ln)
                .iter().map(|x| Type{inner: *x}).collect::<Vec<_>>()
        }
    }
}

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
