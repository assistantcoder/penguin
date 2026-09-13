use cranelift::prelude::{EntityRef};
use pnos_ast;
use cranelift_codegen::ir::{AbiParam, Function, InstBuilder, Signature, UserFuncName, types};
use cranelift_codegen::isa::{CallConv};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift_module::{FuncId, Linkage, Module};
use cranelift_object::{ObjectModule};
use std::collections::HashMap;
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
pub trait MakeNewFunction {
    fn new(&self,module: &mut ObjectModule) -> (FuncId, Function);
}
impl MakeNewFunction for pnos_ast::Function {
    fn new(&self, module: &mut ObjectModule) -> (FuncId, Function) {
        let mut variables = MakeNewVariableDefiner{
            list_vars: HashMap::new(),
            list_values: HashMap::new(),
            index: 0
        };
        let pnos_ast::Function::Declare { name, parametrs, return_type, block } = self;
        let mut sig = Signature::new(CallConv::SystemV);
        if let Some(parametrs) = parametrs {
            for parametr in parametrs.values() {
                match parametr {
                    pnos_ast::TypesKind::Int => sig.params.push(AbiParam::new(types::I64)),
                    pnos_ast::TypesKind::Float => sig.params.push(AbiParam::new(types::F64)),
                    _ => todo!()
                };
            }
        }
        if let Some(v) = return_type {
            match v {
                pnos_ast::TypesKind::Int => sig.returns.push(AbiParam::new(types::I64)),
                pnos_ast::TypesKind::Float => sig.returns.push(AbiParam::new(types::F64)),
                _ => todo!()
            }
        }else {
            sig.returns.push(AbiParam::new(types::I8))
        }
        let func_id = module.declare_function(name.as_str(), Linkage::Export, &sig).unwrap();
        let mut func = Function::with_name_signature(UserFuncName::user(0, func_id.as_u32()), sig);
        let mut fb_ctx = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut func, &mut fb_ctx);
        let block0 = builder.create_block();
        builder.switch_to_block(block0);
        builder.seal_block(block0);
        let pnos_ast::Block::Declare(syntax) = block;
            for rule in syntax {
                match rule {
                    pnos_ast::SyntaxDefiner::Variable(pnos_ast::Variable::Variant { name, valuet }) => {
                        let refrence = variables.declare(name.to_owned(), valuet);
                        builder.declare_var(refrence, 
                            match valuet {
                                pnos_ast::Types::Int(_) => types::I64,
                                pnos_ast::Types::Float(_) => types::F64,
                                _ => todo!()
                            }
                        );
                        let value = match valuet {
                            pnos_ast::Types::Int(v) => &[builder.ins().iconst(types::I64, *v)],
                            pnos_ast::Types::Float(v) => &[builder.ins().f64const(*v)],
                            _ => todo!()
                        };
                        if let Some(var) = variables.list_vars.get(name) {
                            builder.def_var(*var, value[0]);
                        }else {
                            panic!("variable with name {} not found",name);
                        }
                    },
                    pnos_ast::SyntaxDefiner::ReturnKeyword(asigned) => {
                        match asigned {
                            pnos_ast::AllowAsign::Variable(name) =>{
                                if let Some(var) = variables.list_vars.get(name) {
                                    let var_value = builder.use_var(*var);
                                    builder.ins().return_(&[var_value]);
                                }else {
                                    panic!("variable with name {} not found",name);
                                }
                            }
                            _ => todo!()
                        }
                    },
            }
        }
        /*if *return_type == None {
            let exit_code = builder.ins().iconst(types::I8, 0);
            builder.ins().return_(&[exit_code]);
        }*/
        /////////
        builder.finalize();
        (func_id,func)
    }
}