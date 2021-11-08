use crate::basic::{factorial, len, sum, divide};
use num::FromPrimitive;
use num::rational::BigRational;
use num::BigInt;

pub fn npr(num1: BigRational, num2: BigRational) -> BigRational {
    divide(factorial(num1.clone()), factorial(num1-num2))
}

pub fn ncr(num1: BigRational, num2: BigRational) -> BigRational {
    divide(factorial(num1.clone()), factorial(num2.clone())*factorial(num1-num2))
}

pub fn average(nums: Vec<BigRational>) -> BigRational {
    divide(sum(nums.clone()), BigRational::from_usize(len(nums)).unwrap())
}

pub fn median(nums: Vec<BigRational>) -> BigRational {
    todo!();
}
