use lalrpop_util::lalrpop_mod;

use pnos_ast::Void;

lalrpop_mod!(pub grammar, "/grammars/grammar.rs");
pub fn parse(input: String) -> Void{
    let parse_engine = grammar::VoidParser::new();
    let output = parse_engine.parse(&input);
    if let Ok(v) = output{
        return v;
    }else{todo!()}
}