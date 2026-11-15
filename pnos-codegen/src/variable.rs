use crate::function::BuilderWrapper;
use cranelift_codegen::{
    entity::EntityRef,
    ir::{InstBuilder, Value, types},
};
use cranelift_frontend::Variable;
use std::collections::HashMap;

pub struct VariablesLeader<'a> {
    pub var: &'a mut HashMap<String, Variable>,
    index: usize,
}
impl VariablesLeader<'_> {
    pub fn init() -> HashMap<String, Variable> {
        HashMap::new()
    }
    pub fn new(&mut self, name: String, value: pnos_ast::Types, builder: &mut BuilderWrapper) {
        let var = Variable::new(self.index);
        match value {
            pnos_ast::Types::Int(val) => {
                builder.builder.declare_var(var, types::I64);
                let const_for_variable = builder.builder.ins().iconst(types::I64, val);
                builder.builder.def_var(var, const_for_variable);
            }
            pnos_ast::Types::Float(val) => {
                builder.builder.declare_var(var, types::F64);
                let const_for_variable = builder.builder.ins().f64const(val);
                builder.builder.def_var(var, const_for_variable);
            }
            _ => todo!(),
        }
        self.var.insert(name, var);
    }
    pub fn get(&self, key: String) -> Option<&Variable> {
        self.var.get(&key)
    }
}
pub fn variable_matcher(data: pnos_ast::Variable, builder: &mut BuilderWrapper) {
    match data {
        pnos_ast::Variable::New { name, valuet } => builder.var.new(name, valuet, builder),
        _ => todo!(),
    }
}
