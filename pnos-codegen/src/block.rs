use crate::BuilderWrapper;
use std::collections::HashMap;
use cranelift::prelude::EntityRef;
use cranelift_codegen::ir::{InstBuilder, types};
use cranelift_frontend::Variable;
pub fn make_block(builder: &mut BuilderWrapper,block: &pnos_ast::Block){

    let block0 = builder.builder.create_block();
    builder.builder.switch_to_block(block0);
    builder.builder.seal_block(block0);
    let pnos_ast::Block::Declare(syntax) = block;
        for rule in syntax {
            println!("{:?}",rule);
    }
}