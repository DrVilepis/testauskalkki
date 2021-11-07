use libtestauskalkki as ltk;
use std::collections::HashMap;
use std::io::stdin;

const OPERATORS: &str = "+-*/";

struct MathState {
    vars: HashMap<String, Math>,
    curmath: Option<TS>,
    buffer: String,
}

impl MathState {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
            curmath: None,
            buffer: String::new(),
        }
    }
}

type Math = Vec<MathElement>;

#[derive(Debug)]
enum MathElement {
    Operator(MathOperator),
    Number(i64),
    Math(Math),
    Function(MathFunction),
    Variable(String),
}

#[derive(Debug)]
enum MathOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Assign,
}

fn to_math(s: &str) -> MathOperator {
    match s {
        "+" => MathOperator::Add,
        "-" => MathOperator::Subtract,
        "*" => MathOperator::Multiply,
        "/" => MathOperator::Divide,
        "=" => MathOperator::Assign,
        _ => unreachable!("{}", s),
    }
}

#[derive(Debug)]
struct MathFunction {
    func: String,
    args: Math,
}

enum TS {
    Operator,
    Number,
    Math,
    Function,
    Variable,
}

fn main() {
    let stdin = stdin();
    let mut input = String::new();
    let mut math_state = MathState::new();

    loop {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let math = parse_math(&mut input.chars(), &mut math_state);
        println!("Math len: {}, Math: {:?}", math.len(), math);
    }
}

fn parse_math<'a, I>(iter: &mut I, math_state: &mut MathState) -> Math
where
    I: Iterator<Item = char>,
    I: std::fmt::Debug,
{
    let oper: Vec<char> = OPERATORS.chars().collect::<Vec<char>>();
    let mut math = Math::new();
    loop {
        match iter.next() {
            Some(c) => match math_state.curmath {
                Some(ref cm) => match cm {
                    TS::Operator => {
                        if oper.contains(&c) {
                            math_state.buffer.push(c);
                        } else {
                            math.push(MathElement::Operator(to_math(&math_state.buffer)));
                            math_state.curmath = None;
                            start_math(math_state, c);
                        }
                    }
                    TS::Number => {
                        if c.is_ascii_digit() {
                            math_state.buffer.push(c);
                        } else {
                            math_state.curmath = None;
                            math.push(MathElement::Number(
                                match math_state.buffer.parse::<i64>() {
                                    Ok(i) => i,
                                    Err(e) => {
                                        panic!("{}: {}", e, math_state.buffer)
                                    }
                                },
                            ));
                            start_math(math_state, c);
                        }
                    }
                    TS::Math => {
                        math_state.curmath = None;
                        start_math(math_state, c);
                        math.push(MathElement::Math(parse_math(iter, math_state)));
                    }
                    TS::Function => {
                        if c.is_ascii_alphanumeric() {
                            math_state.buffer.push(c);
                        } else if c == '(' {
                            math_state.curmath = None;
                            let name = math_state.buffer.clone();
                            start_math(math_state, c);
                            math.push(MathElement::Function(MathFunction {
                                func: name,
                                args: parse_math(iter, math_state),
                            }));
                        }
                    }
                    TS::Variable => {
                        if c.is_ascii_alphanumeric() {
                            math_state.buffer.push(c);
                        } else if c == '(' {
                            math_state.curmath = None;
                            let name = math_state.buffer.clone();
                            start_math(math_state, c);
                            math.push(MathElement::Function(MathFunction {
                                func: name,
                                args: parse_math(iter, math_state),
                            }));
                        } else {
                            math.push(MathElement::Variable(math_state.buffer.clone()));
                            math_state.buffer.clear();
                            start_math(math_state, c);
                        }
                    }
                },
                None => {
                    start_math(math_state, c);
                }
            },
            None => {
                match math_state.curmath {
                    Some(ref cm) => match cm {
                        TS::Operator => {
                            eprintln!("Math ends with operator");
                        }
                        TS::Number => {
                            math.push(MathElement::Number(
                                math_state.buffer.parse::<i64>().unwrap(),
                            ));
                        }
                        TS::Math => {
                            eprintln!("Math ended too early");
                        }
                        TS::Function => {}
                        TS::Variable => {
                            math.push(MathElement::Variable(math_state.buffer.clone()));
                        }
                    },
                    None => {}
                }
                break;
            }
        };
    }
    math_state.buffer.clear();
    math_state.curmath = None;
    math
}

fn start_math(ms: &mut MathState, c: char) {
    ms.buffer.clear();
    let oper: Vec<char> = OPERATORS.chars().collect::<Vec<char>>();
    ms.curmath = if c.is_ascii_digit() {
        ms.buffer.push(c);
        Some(TS::Number)
    } else if c.is_ascii_alphabetic() {
        ms.buffer.push(c);
        Some(TS::Variable)
    } else if c == '-' {
        ms.buffer.push(c);
        Some(TS::Number)
    } else if oper.clone().contains(&c) {
        ms.buffer.push(c);
        Some(TS::Operator)
    } else if c == '(' {
        Some(TS::Math)
    } else {
        None
    }
}
