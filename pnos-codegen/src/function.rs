use crate::variable::{self, TypesKind};
use cranelift_codegen::ir::{AbiParam, Function, Signature, UserFuncName};
use cranelift_codegen::isa::CallConv;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{FuncId, Linkage, Module};
use cranelift_object::ObjectModule;
use pnos_ast;
pub trait MakeNewFunction {
    fn new(&self, module: &mut ObjectModule) -> (FuncId, Function);
}
pub struct BuilderWrapper<'a> {
    pub builder: FunctionBuilder<'a>,
    pub var: variable::VariablesLeader,
}

impl MakeNewFunction for pnos_ast::Function {
    fn new(&self, module: &mut ObjectModule) -> (FuncId, Function) {
        let pnos_ast::Function::Declare {
            name,
            parametrs,
            return_type,
            block,
        } = self;
        let mut sig = Signature::new(CallConv::SystemV);
        if let Some(parametrs) = parametrs {
            for parametr in parametrs.values() {
                sig.params.push(AbiParam::new(parametr.as_irtype()));
            }
        }
        let rt = return_type;
        sig.returns.push(AbiParam::new(rt.unwrap().as_irtype()));
        let func_id = module
            .declare_function(name.as_str(), Linkage::Export, &sig)
            .unwrap();
        let mut func = Function::with_name_signature(UserFuncName::user(0, func_id.as_u32()), sig);
        let mut fb_ctx = FunctionBuilderContext::new();
        let kind_builder = FunctionBuilder::new(&mut func, &mut fb_ctx);
        let mut builder = BuilderWrapper {
            builder: kind_builder,
            var: variable::VariablesLeader::init(),
        };
        crate::make_block(&mut builder, block);
        /////////
        builder.builder.finalize();
        (func_id, func)
    }
}
