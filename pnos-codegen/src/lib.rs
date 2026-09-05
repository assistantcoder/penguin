use cranelift_codegen::ir::{AbiParam, Function, InstBuilder, Signature, types,UserFuncName};
use cranelift_codegen::isa::{CallConv};
use cranelift_codegen::{settings, Context};
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{default_libcall_names, Module, ModuleError, Linkage};
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;
use std::fs;
use pnos_ast;
struct AotCompiler {
    module: ObjectModule,
}
impl AotCompiler {
    fn new() -> Result<Self, ModuleError> {
        let isa_builder = cranelift_codegen::isa::lookup(Triple::host()).unwrap();
        let isa = isa_builder.finish(settings::Flags::new(settings::builder()))?;
        let obj_builder = ObjectBuilder::new(isa, "prog", default_libcall_names())?;
        let module = ObjectModule::new(obj_builder);
        Ok(Self { module })
    }
    fn write(self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let product = self.module.finish();
        let bytes = product.emit()?;
        fs::write(path, bytes)?;
        Ok(())
    }
}
fn exec() -> AotCompiler{
    
    
    let mut compiler = AotCompiler::new().unwrap();
    let module = &mut compiler.module;
    let mut sig = Signature::new(CallConv::SystemV);
    sig.params.push(AbiParam::new(types::I32));   // أول parameter
    sig.params.push(AbiParam::new(types::I32));   // ثاني parameter
    sig.returns.push(AbiParam::new(types::I32));  // نوع الإرجاع
    let func_id = module.declare_function("main", Linkage::Export, &sig).unwrap();
    let mut func = Function::with_name_signature(UserFuncName::user(0, func_id.as_u32()), sig);
    let mut fb_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut func, &mut fb_ctx);
    let block0 = builder.create_block();                          // أنشئ block
    builder.append_block_params_for_function_params(block0);      // خله ياخذ params الدالة
    builder.switch_to_block(block0);                                // ابدأ تكتب هنا
    builder.seal_block(block0);
    let v0 = builder.block_params(block0)[0];
    let v1 = builder.block_params(block0)[1];

    let sum = builder.ins().iadd(v0, v1);
    builder.ins().return_(&[sum]);
    builder.finalize();
    let mut ctx = Context::for_function(func);
    module.define_function(func_id, &mut ctx).unwrap();
    module.clear_context(&mut ctx);
    compiler
}
pub fn codegen(ast: pnos_ast::Void) {
    let compiler = exec();
    compiler.write("main.o").unwrap();
    println!("✓ main.o created");
    println!("{ast:?}")
}
