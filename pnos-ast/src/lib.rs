use std::collections::HashMap;
#[derive(Debug)]
pub enum Block{
    Declare(Vec<SyntaxDefiner>)
}
#[derive(Debug)]
pub enum Void {
    Function(Function)
}
#[derive(Debug)]
pub enum SyntaxDefiner {
    Variable(Variable),
    ReturnKeyword(AllowAsign)
}
//This section for calculate and operations
#[derive(Debug,PartialEq)]
pub enum Oprators{
    Add,
    Sub,
    Div,
    Mul
}
#[derive(Debug,PartialEq)]
pub enum CalculateAst {
    Number(i64),
    ExprBinary{
        left: Box<CalculateAst>,
        op: Oprators,
        right: Box<CalculateAst>
    }
}
//This section for variable, types and assignment
#[derive(Debug)]
pub enum AllowAsign {
    Variable(String),
    Value(Types)
}

#[derive(Debug,PartialEq,Clone)]
pub enum Types{
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool)
}
#[derive(Debug,PartialEq, Eq)]
pub enum TypesKind{
    Int,
    Float,
    String,
    Bool
}
#[derive(Debug,PartialEq)]
pub enum Variable {
    Variant{name: String, valuet: Types}
}
impl Types {
    pub fn kind(&self) -> TypesKind {
        match self {
            Types::Int(_) => TypesKind::Int,
            Types::Float(_) => TypesKind::Float,
            Types::String(_) => TypesKind::String,
            Types::Bool(_) => TypesKind::Bool
        }
    }
}
//========functions sction==============
#[derive(Debug)]
pub enum Function{
    Declare{name: String,parametrs: Option<HashMap<String,TypesKind>>,return_type: Option<TypesKind>,block: Block}
}
