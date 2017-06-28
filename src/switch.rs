use std::convert::{From,TryFrom};

use basic_block::BasicBlock;
use value::{Value,Opcode};
use bindings::*;

#[derive(PartialEq,Eq,Copy,Clone)]
pub struct Switch(pub(super) LLVMValueRef);

impl Switch {
    pub fn default_dest(&self) -> BasicBlock {
        BasicBlock(unsafe { LLVMGetSwitchDefaultDest(self.0) })
    }
}

impl From<Switch> for Value {
    fn from(other: Switch) -> Value {
        Value(other.0)
    }
}

impl TryFrom<Value> for Switch {
    type Error = ();
    fn try_from(other: Value) -> Result<Switch, ()> {
        if other.opcode() == Opcode::LLVMSwitch {
            Ok(Switch(other.0))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use builder::IRBuilder;
    use value::Value;
    use types::{Type,FunctionType};
    use module::Module;
    use std::collections::HashMap;

    #[test]
    fn test_default_dest() {
        let modl = Module::new("test");
        let f = modl.add_function("testf", FunctionType::new(Type::int32(), &vec![], false));

        let _ = f.append_bb("entry");
        let next_b = f.append_bb("other");
        let nnext_b = f.append_bb("nnext");

        let builder = IRBuilder::new();
        builder.position_at_end(f.entry_bb().unwrap());
        let mut hash = HashMap::new();
        hash.insert(Value::const_int(Type::int1(), 0), nnext_b);

        let sw = builder.switch(Value::const_int(Type::int1(), 1), next_b, hash);
        assert_eq!(next_b, sw.default_dest());

    }
}
