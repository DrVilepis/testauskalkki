mod eval;
mod mathstructs;
use std::io::stdin;

use mathstructs::*;

fn main() {
    let stdin = stdin();
    let mut input = String::new();
    let mut math_state = MathState::new();

    loop {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let math = parse_math(&mut input.chars(), &mut math_state);
        if let Some(ans) = eval::eval_math(&mut math_state.vars, math) {
            println!("= {}", ans);
        }
    }
}

fn parse_math<'a, I>(iter: &mut I, math_state: &mut MathState) -> Math
where
    I: Iterator<Item = char> + std::fmt::Debug,
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
                        } else if c.is_ascii_digit() {
                            let tmpc = math_state.buffer.pop().unwrap();
                            if tmpc == '-' {
                                math.push(MathElement::Operator(to_math(&math_state.buffer)));
                                math_state.buffer.clear();
                                math_state.buffer.push(tmpc);
                                math_state.buffer.push(c);
                                math_state.curmath = Some(TS::Number);
                            } else {
                                math_state.buffer.push(tmpc);
                                math.push(MathElement::Operator(to_math(&math_state.buffer)));
                                start_math(math_state, c);
                            };
                        } else {
                            math.push(MathElement::Operator(to_math(&math_state.buffer)));
                            start_math(math_state, c);
                        }
                    }
                    TS::Number => {
                        if c.is_ascii_digit() || c == '.' || c == ',' {
                            math_state.buffer.push(c);
                        } else {
                            math_state.curmath = None;
                            if math_state.buffer == "-" {
                                math.push(MathElement::Operator(to_math(&math_state.buffer)));
                            } else {
                                math.push(MathElement::Number(parse_int(&math_state.buffer)));
                            }
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
                            math.push(MathElement::Number(parse_int(&math_state.buffer)));
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

fn parse_int(s: &str) -> f64 {
    s.parse::<f64>().unwrap()
}
fn exec_func(fn_name: &str,args: Vec<f64>) -> f64 {
    todo!();
}
