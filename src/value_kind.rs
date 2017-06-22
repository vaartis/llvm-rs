use ::value::*;

#[allow(dead_code)]
extern "C" {
    fn LLVMValueIsNullValue(val: *const CValue) -> bool;
    fn LLVMValueIsArgument(val: *const CValue) -> bool;
 	fn LLVMValueIsBasicBlock(val: *const CValue) -> bool;
 	fn LLVMValueIsInlineAsm(val: *const CValue) -> bool;
 	fn LLVMValueIsMDNode(val: *const CValue) -> bool;
 	fn LLVMValueIsMDString(val: *const CValue) -> bool;
 	fn LLVMValueIsBlockAddress(val: *const CValue) -> bool;
 	fn LLVMValueIsConstantAggregateZero(val: *const CValue) -> bool;
 	fn LLVMValueIsConstantArray(val: *const CValue) -> bool;
 	fn LLVMValueIsConstantDataArray(val: *const CValue) -> bool;
 	fn LLVMValueIsConstantDataVector(val: *const CValue) -> bool;
 	fn LLVMValueIsConstantExpr(val: *const CValue) -> bool;
 	fn LLVMValueIsConstantFP(val: *const CValue) -> bool;
 	fn LLVMValueIsConstantInt(val: *const CValue) -> bool;
 	fn LLVMValueIsConstantPointerNull(val: *const CValue) -> bool;
 	fn LLVMValueIsConstantStruct(val: *const CValue) -> bool;
 	fn LLVMValueIsConstantVector(val: *const CValue) -> bool;
 	fn LLVMValueIsFunction(val: *const CValue) -> bool;
 	fn LLVMValueIsGlobalAlias(val: *const CValue) -> bool;
 	fn LLVMValueIsGlobalVariable(val: *const CValue) -> bool;
 	fn LLVMValueIsUndefValue(val: *const CValue) -> bool;
}

#[derive(PartialEq, Eq)]
pub enum ValueKind {
    NullValue,
    Argument,
 	BasicBlock,
 	InlineAsm,
 	MDNode,
 	MDString,
 	BlockAddress,
 	ConstantAggregateZero,
 	ConstantArray,
 	ConstantDataArray,
 	ConstantDataVector,
 	ConstantExpr,
 	ConstantFP,
 	ConstantInt,
 	ConstantPointerNull,
 	ConstantStruct,
 	ConstantVector,
 	Function,
 	GlobalAlias,
 	GlobalVariable,
 	UndefValue
}

macro_rules! classify_case {
    ($($val:expr, $kind:ident),+) => {
        $(
            if concat_idents!(LLVMValueIs, $kind)($val) {
                return ValueKind::$kind;
            }
        )+
    }
}

impl Value {
    pub fn classify(&self) -> ValueKind {
        unsafe {
            classify_case!(
                self.0, BasicBlock
            );
        }
        unreachable!();
    }
}
