use value::*;
use bindings::*;

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
            if concat_idents!(LLVMValueIs, $kind)($val) == 1 {
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
