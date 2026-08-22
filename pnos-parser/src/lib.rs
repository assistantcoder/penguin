use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar, "/grammars/grammar.rs");
pub mod ast;

pub fn parse(input: &'static str){
    let parse_engine = grammar::VoidParser::new();
    let output = parse_engine.parse(input);
    println!("{output:?}");
}