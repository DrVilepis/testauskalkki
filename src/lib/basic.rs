use num::bigint::Sign;
use num::{BigInt, FromPrimitive};

pub fn add(num1: f64, num2: f64) -> f64 {
    num1 + num2
}

pub fn subtract(num1: f64, num2: f64) -> f64 {
    num1 - num2
}

pub fn inv(num: f64) -> f64 {
    -num
}

pub fn abs(num: f64) -> f64 {
    num.abs()
}

pub fn multiply(num1: f64, num2: f64) -> f64 {
    num1 * num2
}

pub fn divide(num1: f64, num2: f64) -> f64 {
    num1 / num2
}

pub fn intdivide(num1: f64, num2: f64) -> f64 {
    divide(num1, num2).round()
}

pub fn sum(nums: Vec<f64>) -> f64 {
    nums.iter().sum()
}

pub fn len(nums: Vec<f64>) -> usize {
    nums.len()
}

pub fn factorial(num: f64) -> f64 {
    num.clone() * factorial(num - 1.)
}

pub fn floor(num: f64) -> f64 {
    num.floor()
}

pub fn ceil(num: f64) -> f64 {
    num.ceil()
}

pub fn round(num: f64) -> f64 {
    num.round()
}

pub fn pow(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

pub fn log(_base: f64, _numerus: f64) {
    todo!();
}

//pub fn min(nums: Vec<f64>) -> f64 {
//    nums.iter().min().unwrap().to_owned()
//}

//pub fn max(nums: Vec<f64>) -> f64 {
//    nums.iter().max().unwrap().to_owned()
//}

pub fn modulo(num1: f64, num2: f64) -> f64 {
    num1 % num2
}

pub fn factor(mut num: f64) -> Vec<f64> {
    let mut ret = Vec::new();
    let inc = vec![1.0, 2.0, 2.0, 4.0, 2.0, 4.0, 2.0, 4.0, 6.0, 2.0, 6.0];
    let mut k = 2f64;
    let mut i = 0;
    while multiply(k, k) <= num {
        if modulo(num, k) == 0.0 {
            ret.push(k);
            num = divide(num, k);
        } else {
            k += inc[i];
            if i < 10 {
                i += 1;
            } else {
                i = 3
            }
        }
    }
    if num > 1.0 {
        ret.push(num);
    }
    ret
}
