use crate::function::BuilderWrapper;
use cranelift_codegen::{
    entity::EntityRef,
    ir::{InstBuilder, Value, types},
};
use cranelift_frontend::{FunctionBuilder, Variable};
use std::collections::HashMap;

trait AstTypes {
    fn extract_value(&self, builder: &mut FunctionBuilder) -> Value;
}
trait TypesKind {
    fn as_irtype(&self) -> types::Type;
}
impl AstTypes for pnos_ast::Types {
    fn extract_value(&self, builder: &mut FunctionBuilder) -> Value {
        match self {
            Self::Int(v) => builder.ins().iconst(types::I64, *v),
            Self::Float(v) => builder.ins().f64const(*v),
            _ => todo!(),
        }
    }
}
impl TypesKind for pnos_ast::TypesKind {
    fn as_irtype(&self) -> types::Type {
        match self {
            Self::Int => types::I64,
            Self::Float => types::F64,
            _ => todo!(),
        }
    }
}
#[derive(Debug)]
pub struct VariablesLeader {
    pub var: HashMap<String, (pnos_ast::TypesKind, Variable)>,
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
        println!("--------{:?}-------", value);
        let const_for_variable = value.extract_value(builder);
        builder.declare_var(var, value.kind().as_irtype());
        builder.def_var(var, const_for_variable);
        self.var.insert(name, (value.kind(), var));
        self.index += 1;
    }
    pub fn get(&self, key: &str) -> Option<&(pnos_ast::TypesKind, Variable)> {
        self.var.get(key)
    }
}
pub fn variable_matcher(data: &pnos_ast::Variable, builder: &mut BuilderWrapper) {
    match data {
        pnos_ast::Variable::Declare(name, pnos_ast::AllowAsign::Value(valuet)) => {
            builder
                .var
                .new(name.to_string(), valuet.clone(), &mut builder.builder)
        }
        pnos_ast::Variable::FromOld(new, pnos_ast::AllowAsign::Variable(old)) => {
            println!("{}", new);
            println!("{:?}----------------------", builder.var);
            let Some(var) = builder.var.get(old) else {
                todo!()
            };
            let const_for_variale = builder.builder.use_var(var.1);
            let new_var = Variable::new(builder.var.index);
            builder.builder.declare_var(new_var, var.0.as_irtype());
            builder.builder.def_var(new_var, const_for_variale);
            builder.var.var.insert(new.to_string(), (var.0, new_var));
            builder.var.index += 1;
            println!("{:?}----------------------", builder.var);
        }
        _ => todo!(),
    }
}
