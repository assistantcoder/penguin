use crate::{BuilderWrapper, variable};
use cranelift_codegen::ir::InstBuilder;
use variable::AstTypes;
pub fn match_next(builder: &mut BuilderWrapper, block: &pnos_ast::Block) {
    let pnos_ast::Block::Declare(syntaxs) = block;
    for rule in syntaxs {
        println!("rule {:?}", rule);
        match rule {
            //define return keyword to run
            pnos_ast::SyntaxDefiner::ReturnKeyword(pnos_ast::AllowAsign::Value(v)) => {
                let re = v.extract_value(&mut builder.builder);
                builder.builder.ins().return_(&[re]);
            }
            //define variable
            pnos_ast::SyntaxDefiner::Variable(v) => variable::variable_matcher(v, builder),
            pnos_ast::SyntaxDefiner::ReturnKeyword(pnos_ast::AllowAsign::Variable(v)) => {
                if let Some(var) = builder.var.get(v) {
                    let re = builder.builder.use_var(var.1);
                    builder.builder.ins().return_(&[re]);
                }
            }
        }
    }
}
