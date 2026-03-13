use std::collections::HashMap;

#[derive(Debug)]
pub enum Block {
    Declare(Vec<SyntaxDefiner>),
}
#[derive(Debug)]
pub enum Void {
    Function(Function),
}
#[derive(Debug)]
pub enum SyntaxDefiner {
    Variable(Variable),
    ReturnKeyword(AllowAsign),
}
//This section for calculate and operations
#[derive(Debug, PartialEq)]
pub enum Oprators {
    Add,
    Sub,
    Div,
    Mul,
}
#[derive(Debug, PartialEq)]
pub enum CalculateAst {
    Number(i64),
    ExprBinary {
        left: Box<CalculateAst>,
        op: Oprators,
        right: Box<CalculateAst>,
    },
}
//This section for variable, types and assignment
#[derive(Debug, PartialEq)]
pub enum AllowAsign {
    Variable(String),
    Value(Types),
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Types {
    Int(i64),
    Float(f64),
    Bool(bool),
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TypesKind {
    Int,
    Float,
    Bool,
}
#[derive(Debug, PartialEq)]
pub enum Variable {
    Declare(String, AllowAsign),
    FromOld(String, AllowAsign),
}
impl Types {
    pub fn kind(&self) -> TypesKind {
        match self {
            Types::Int(_) => TypesKind::Int,
            Types::Float(_) => TypesKind::Float,
            Types::Bool(_) => TypesKind::Bool,
        }
    }
}

//========functions sction==============
#[derive(Debug)]
pub enum Function {
    Declare {
        name: String,
        parametrs: Option<HashMap<String, TypesKind>>,
        return_type: Option<TypesKind>,
        block: Block,
    },
}
