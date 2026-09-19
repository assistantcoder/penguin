use crate::BuilderWrapper;
use std::collections::HashMap;
use cranelift::prelude::EntityRef;
use cranelift_codegen::ir::{AbiParam, Function, InstBuilder, Signature, UserFuncName, types};
use cranelift_codegen::isa::{CallConv};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext,Variable};
use cranelift_module::{FuncId, Linkage, Module};
use cranelift_object::{ObjectModule};
struct MakeNewVariableDefiner{
    list_vars: HashMap<String,Variable>,
    list_values: HashMap<String,pnos_ast::Types>,
    index: usize
}
impl MakeNewVariableDefiner{
    fn declare(&mut self,key: String,value: &pnos_ast::Types) -> Variable{
        let var = Variable::new(self.index);
        let key = key;
        self.index += 1;
        let _ = &self.list_vars.insert(key.clone(),var);
        let _ = self.list_values.entry(key).or_insert(value.clone());
        var
    }
}
pub fn make_block(builder: &mut BuilderWrapper,block: &pnos_ast::Block){
    let mut variables = MakeNewVariableDefiner{
        list_vars: HashMap::new(),
        list_values: HashMap::new(),
        index: 0
    };
    let block0 = builder.builder.create_block();
    builder.builder.switch_to_block(block0);
    builder.builder.seal_block(block0);
    let pnos_ast::Block::Declare(syntax) = block;
        for rule in syntax {
            match rule {
                pnos_ast::SyntaxDefiner::Variable(pnos_ast::Variable::Variant { name, valuet }) => {
                    let refrence = variables.declare(name.to_owned(), valuet);
                    builder.builder.declare_var(refrence, 
                        match valuet {
                            pnos_ast::Types::Int(_) => types::I64,
                            pnos_ast::Types::Float(_) => types::F64,
                            _ => todo!()
                        }
                    );
                    let value = match valuet {
                        pnos_ast::Types::Int(v) => &[builder.builder.ins().iconst(types::I64, *v)],
                        pnos_ast::Types::Float(v) => &[builder.builder.ins().f64const(*v)],
                        _ => todo!()
                    };
                    if let Some(var) = variables.list_vars.get(name) {
                        builder.builder.def_var(*var, value[0]);
                    }else {
                        panic!("variable with name {} not found",name);
                    }
                },
                pnos_ast::SyntaxDefiner::ReturnKeyword(asigned) => {
                    match asigned {
                        pnos_ast::AllowAsign::Variable(name) =>{
                            if let Some(var) = variables.list_vars.get(name) {
                                let var_value = builder.builder.use_var(*var);
                                builder.builder.ins().return_(&[var_value]);
                            }else {
                                panic!("variable with name {} not found",name);
                            }
                        }
                        _ => todo!()
                    }
                },
        }
    }
}
