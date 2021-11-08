pub mod basic;
pub mod statistical;
use num::FromPrimitive;
use num::bigint::BigInt;
use num::rational::{Ratio, BigRational};

pub fn approx_sqrt(number: u64, iterations: usize) -> BigRational {
    let start: Ratio<BigInt> = Ratio::from_integer(FromPrimitive::from_u64(number).unwrap());
    let mut approx = start.clone();

    for _ in 0..iterations {
        approx = (&approx + (&start / &approx)) /
            Ratio::from_integer(FromPrimitive::from_u64(2).unwrap());
    }

    approx
}

pub fn add(num1: u64, num2: u64) -> u64 {
    num1 + num2
}


