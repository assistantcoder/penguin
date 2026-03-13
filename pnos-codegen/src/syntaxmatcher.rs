use crate::{BuilderWrapper, variable};
use cranelift_codegen::ir::{InstBuilder, types};
pub fn match_next(builder: &mut BuilderWrapper, block: &pnos_ast::Block) {
    let pnos_ast::Block::Declare(syntaxs) = block;
    for rule in syntaxs {
        println!("rule {:?}", rule);
        match rule {
            //define return keyword to run
            pnos_ast::SyntaxDefiner::ReturnKeyword(pnos_ast::AllowAsign::Value(v)) => match v {
                pnos_ast::Types::Int(v) => {
                    let re = builder.builder.ins().iconst(types::I64, *v);
                    builder.builder.ins().return_(&[re]);
                }
                pnos_ast::Types::Float(v) => {
                    let re = builder.builder.ins().f64const(*v);
                    builder.builder.ins().return_(&[re]);
                }
                pnos_ast::Types::String(_) | pnos_ast::Types::Bool(_) => todo!(),
            },
            //define variable
            pnos_ast::SyntaxDefiner::Variable(v) => variable::variable_matcher(v, builder),
            pnos_ast::SyntaxDefiner::ReturnKeyword(pnos_ast::AllowAsign::Variable(v)) => {
                if let Some(var) = builder.var.get(v.to_string()) {
                    let re = builder.builder.use_var(var.to_owned());
                    builder.builder.ins().return_(&[re]);
                }
            }
        }
    }
}
