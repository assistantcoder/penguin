use crate::BuilderWrapper;
pub fn make_block(builder: &mut BuilderWrapper,block: &pnos_ast::Block){

    let block0 = builder.builder.create_block();
    builder.builder.switch_to_block(block0);
    builder.builder.seal_block(block0);
    crate::syntaxmatcher::match_next(builder,block);
}