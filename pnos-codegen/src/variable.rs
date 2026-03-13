use crate::function::BuilderWrapper;
use cranelift_codegen::{
    entity::EntityRef,
    ir::{InstBuilder, types},
};
use cranelift_frontend::{FunctionBuilder, Variable};
use std::collections::HashMap;

pub struct VariablesLeader {
    pub var: HashMap<String, Variable>,
    index: usize,
}
impl VariablesLeader {
    pub fn init() -> VariablesLeader {
        VariablesLeader {
            var: HashMap::new(),
            index: 0,
        }
    }
    pub fn new(&mut self, name: String, value: pnos_ast::Types, builder: &mut FunctionBuilder) {
        let var = Variable::new(self.index);
        match value {
            pnos_ast::Types::Int(val) => {
                builder.declare_var(var, types::I64);
                let const_for_variable = builder.ins().iconst(types::I64, val);
                builder.def_var(var, const_for_variable);
            }
            pnos_ast::Types::Float(val) => {
                builder.declare_var(var, types::F64);
                let const_for_variable = builder.ins().f64const(val);
                builder.def_var(var, const_for_variable);
            }
            _ => todo!(),
        }
        self.var.insert(name, var);
        self.index += 1;
    }
    pub fn get(&self, key: String) -> Option<&Variable> {
        self.var.get(&key)
    }
}
pub fn variable_matcher(data: &pnos_ast::Variable, builder: &mut BuilderWrapper) {
    match data {
        pnos_ast::Variable::Declare(name, pnos_ast::AllowAsign::Value(valuet)) => {
            builder
                .var
                .new(name.to_string(), valuet.clone(), &mut builder.builder)
        }
        pnos_ast::Variable::FromOld(old, pnos_ast::AllowAsign::Variable(new)) => {
            let Some(var) = builder.var.get(old.to_string()) else {
                todo!()
            };
            builder.builder.use_var(var);
        }
        _ => todo!(),
    }
}
