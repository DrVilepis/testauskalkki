use std::collections::HashMap;

pub struct MathState {
    pub vars: HashMap<String, Math>,
    pub curmath: Option<TS>,
    pub buffer: String,
}

impl MathState {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            curmath: None,
            buffer: String::new(),
        }
    }
}

pub type Math = Vec<MathElement>;

#[derive(Clone, Debug)]
pub enum MathElement {
    Operator(MathOperator),
    Number(f64),
    Math(Math),
    Function(MathFunction),
    Variable(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MathOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Assign,
    Power,
}

pub const OPERATORS: &str = "+-*/=;^";

pub fn to_math(s: &str) -> MathOperator {
    match s {
        "+" => MathOperator::Add,
        "-" => MathOperator::Subtract,
        "*" => MathOperator::Multiply,
        "/" => MathOperator::Divide,
        "=" => MathOperator::Assign,
        "^" => MathOperator::Power,
        _ => panic!("Invalid operator: {}", s),
    }
}

#[derive(Clone, Debug)]
pub struct MathFunction {
    pub func: String,
    pub args: Math,
}

pub enum TS {
    Operator,
    Number,
    Math,
    Function,
    Variable,
}
