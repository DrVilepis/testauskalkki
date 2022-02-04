use std::collections::HashMap;

use libtestauskalkki as ltk;
use mathstructs::MathElement::Number;

use crate::*;

pub fn eval_math(vars: &mut HashMap<String, Math>, math: Math) -> Option<f64> {
    //dbg!("{}",&math);
    let mut math = math.iter();
    let mut math_buffer = Math::new();
    while let Some(m) = math.next() {
        match m {
            MathElement::Operator(o) if o == &MathOperator::Assign => {
                let var = math_buffer.pop().unwrap();
                match var {
                    MathElement::Variable(s) => {
                        vars.insert(s, math.clone().map(|e| e.to_owned()).collect());
                        return None;
                    }
                    _ => eprintln!("Skill"),
                }
            }
            _ => math_buffer.push(m.clone()),
        }
    }
    let tmp = math_buffer.clone();
    math = tmp.iter();
    math_buffer.clear();
    while let Some(m) = math.next() {
        match m {
            MathElement::Math(_) => {
                let math = reduce(vars, m);
                math_buffer.push(mathstructs::MathElement::Number(math));
            }
            MathElement::Function(_) => {}
            _ => math_buffer.push(m.clone()),
        }
    }
    let tmp = math_buffer.clone();
    math = tmp.iter();
    math_buffer.clear();
    while let Some(m) = math.next() {
        match m {
            MathElement::Number(_) => math_buffer.push(m.clone()),
            MathElement::Operator(o) => match o {
                MathOperator::Multiply | MathOperator::Divide => {
                    let m1 = math_buffer.pop().unwrap();
                    math_buffer.push(Number(operate(
                        o,
                        reduce(vars, &m1),
                        reduce(vars, math.next().unwrap()),
                    )));
                }
                _ => math_buffer.push(m.clone()),
            },
            MathElement::Function(_) => {}
            _ => math_buffer.push(m.clone()),
        }
    }
    let tmp = math_buffer.clone();
    math = tmp.iter();
    math_buffer.clear();
    while let Some(m) = math.next() {
        match m {
            MathElement::Operator(o) => {
                let m1 = math_buffer.pop().unwrap();
                math_buffer.push(Number(operate(
                    o,
                    reduce(vars, &m1),
                    reduce(vars, math.next().unwrap()),
                )));
            }
            _ => math_buffer.push(m.clone()),
        }
    }
    match math_buffer.pop().unwrap() {
        MathElement::Number(n) => Some(n),
        MathElement::Variable(v) => Some(reduce(vars, &MathElement::Variable(v))),
        _ => {
            eprintln!("Error: Missing skill");
            None
        }
    }
}

pub fn reduce(vars: &mut HashMap<String, Math>, math: &MathElement) -> f64 {
    match math {
        MathElement::Number(n) => n.to_owned(),
        MathElement::Operator(_o) => {
            panic!("Skill issue");
        }
        MathElement::Math(m) => eval_math(vars, m.clone()).unwrap(),
        MathElement::Variable(var) => eval_math(vars, vars.get(var).unwrap().to_vec()).unwrap(),
        _ => todo!("{:?}", math),
    }
}

pub fn operate(oper: &MathOperator, n1: f64, n2: f64) -> f64 {
    match oper {
        MathOperator::Add => ltk::basic::add(n1, n2),
        MathOperator::Subtract => ltk::basic::subtract(n1, n2),
        MathOperator::Multiply => ltk::basic::multiply(n1, n2),
        MathOperator::Divide => ltk::basic::divide(n1, n2),
        MathOperator::Power => ltk::basic::pow(n1, n2),
        _ => todo!(),
    }
}
