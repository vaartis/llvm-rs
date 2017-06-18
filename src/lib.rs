extern crate libc;

use std::ffi::{CString, CStr};
use std::fmt;

enum CModule {}

pub struct Module {
    pub name: String,
    inner: *const CModule
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

enum CType {}

pub struct Type {
    inner: *const CType
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

    pub fn int8() -> Type { Type{inner: unsafe { LLVMInt8Type() }} }
    pub fn int16() -> Type { Type{inner: unsafe { LLVMInt16Type() }} }
    pub fn int32() -> Type { Type{inner: unsafe { LLVMInt32Type() }} }
    pub fn int64() -> Type { Type{inner: unsafe { LLVMInt64Type() }} }
    pub fn int128() -> Type { Type{inner: unsafe { LLVMInt128Type() }} }
    pub fn int(num: libc::c_uint) -> Type { Type{inner: unsafe { LLVMIntType(num) }} } // c_uint is just an alias, so probably OK

    pub fn function_type(ret_type: Type, args: &[Type], is_vararg: bool) -> Type {
        let c_ret_t = ret_type.inner;
        let args_ctypes = args.iter().map(|x| x.inner).collect::<Vec<_>>().as_ptr() as *const CType;
        Type{inner: unsafe { LLVMFunctionType(c_ret_t, args_ctypes, args.len() as libc::c_uint, is_vararg) }}
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c_s = unsafe { CStr::from_ptr(LLVMPrintTypeToString(self.inner)) };
        let st = c_s.to_str().unwrap(); // Propably valid utf8
        write!(f, "{}", st)
    }
}

#[link(name = "LLVM-4.0")]
extern "C" {
    fn LLVMModuleCreateWithName(s: *const libc::c_char) -> *const CModule;
    fn LLVMPrintModuleToString(m: *const CModule) -> *const libc::c_char;
    fn LLVMDisposeModule(m: *const CModule);

    fn LLVMInt8Type() -> *const CType;
    fn LLVMInt16Type() -> *const CType;
    fn LLVMInt32Type() -> *const CType;
    fn LLVMInt64Type() -> *const CType;
    fn LLVMInt128Type() -> *const CType;
    fn LLVMIntType(num: libc::c_uint) -> *const CType;

    fn LLVMGetTypeKind(tp: *const CType) -> TypeKind;
    fn LLVMFunctionType(ret_type: *const CType , args: *const CType, param_count: libc::c_uint, is_vararg: bool) -> *const CType;
    fn LLVMPrintTypeToString(tp: *const CType) -> *const libc::c_char;
}

#[test]
fn test_type_kind() {
    assert!(Type::int32().kind() == TypeKind::Integer);
    assert!(Type::function_type(Type::int32(), &vec![], false).kind() == TypeKind::Function);
}
