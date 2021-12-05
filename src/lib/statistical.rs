use num::FromPrimitive;

use crate::basic::{divide, factorial, len, sum};

pub fn npr(num1: f64, num2: f64) -> f64 {
    divide(factorial(num1.clone()), factorial(num1 - num2))
}

pub fn ncr(num1: f64, num2: f64) -> f64 {
    divide(
        factorial(num1.clone()),
        factorial(num2.clone()) * factorial(num1 - num2),
    )
}

pub fn average(nums: Vec<f64>) -> f64 {
    divide(sum(nums.clone()), f64::from_usize(len(nums)).unwrap())
}

pub fn median(_nums: Vec<f64>) -> f64 {
    todo!();
}
