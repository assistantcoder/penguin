use pnos_ast;
use cranelift_codegen::ir::{AbiParam, Function, InstBuilder, Signature, UserFuncName, types};
use cranelift_codegen::isa::{CallConv};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{FuncId, Linkage, Module};
use cranelift_object::{ObjectModule};

pub trait MakeNewFunction {
    fn new(&self,module: &mut ObjectModule) -> (FuncId, Function);
}
pub struct BuilderWrapper<'a>{
    pub builder: FunctionBuilder<'a>
}

impl MakeNewFunction for pnos_ast::Function {
    fn new(&self, module: &mut ObjectModule) -> (FuncId, Function) {

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
        let mut kind_builder = FunctionBuilder::new(&mut func, &mut fb_ctx);
        let mut builder = BuilderWrapper { builder: kind_builder };
        crate::make_block(&mut builder,block);
        if *return_type == None {
            let exit_code = builder.builder.ins().iconst(types::I8, 0);
            builder.builder.ins().return_(&[exit_code]);
        }
        /////////
        builder.builder.finalize();
        (func_id,func)
    }
}
