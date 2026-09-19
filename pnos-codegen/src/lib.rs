use pnos_ast;
mod function;
mod helper;
mod block;
use function::{MakeNewFunction,BuilderWrapper};
use block::make_block;
use helper::{AotCompiler};
use cranelift_codegen::Context;
use cranelift_module::Module;
fn exec(ast: pnos_ast::Void) -> AotCompiler{
    let mut compiler = AotCompiler::new().unwrap();
    let module = &mut compiler.module;
    match ast {
        pnos_ast::Void::Function(pnos_ast::Function::Declare { name, parametrs, return_type, block }) => {
            let func = pnos_ast::Function::Declare { name, parametrs, return_type, block }.new(module);
            let mut ctx = Context::for_function(func.1.clone());
            println!("----------ir--------------");
            println!("{:?}",func.1);
            module.define_function(func.0, &mut ctx).unwrap();
            module.clear_context(&mut ctx);
        }
    }
/*
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
    */
    compiler
}
pub fn codegen(ast: pnos_ast::Void) {
    println!("{:?}",&ast);
    let compiler = exec(ast);
    compiler.write("main.o").unwrap();
    println!("✓ main.o created");
}
