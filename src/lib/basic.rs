use num::FromPrimitive;
use num::rational::BigRational;
use num::BigInt;
use num::bigint::Sign;

fn as_bigint(num: u64) -> BigRational {
    BigRational::from_integer(BigInt::from_u64(num).unwrap())
}

pub fn add(num1: BigRational, num2: BigRational) -> BigRational {
    num1 + num2
}

pub fn subtract(num1: BigRational, num2: BigRational) -> BigRational {
    num1 - num2
}

pub fn inv(num: BigRational) -> BigRational {
    -num
}

pub fn abs(num: BigRational) -> BigRational {
    let mut signs = false;
    if let Sign::Minus = num.numer().sign() {
        signs = !signs;
    }
    if let Sign::Minus = num.denom().sign() {
        signs = !signs;
    }
    if signs {
        return inv(num);
    }
    num
}

pub fn multiply(num1: BigRational, num2: BigRational) -> BigRational {
    num1 * num2
}

pub fn divide(num1: BigRational, num2: BigRational) -> BigRational {
    num1 / num2
}

pub fn intdivide(num1: BigRational, num2: BigRational) -> BigRational {
    divide(num1, num2).round()
}

pub fn sum(nums: Vec<BigRational>) -> BigRational {
    nums.iter().sum()
}

pub fn len(nums: Vec<BigRational>) -> usize {
    nums.len()
}

pub fn factorial(num: BigRational) -> BigRational {
    if num == as_bigint(1) || num == as_bigint(0) {
        return as_bigint(1)
    }
    num.clone()*factorial(num-as_bigint(1))
}

pub fn floor(num: BigRational) -> BigRational {
    num.floor()
}

pub fn ceil(num: BigRational) -> BigRational {
    num.ceil()
}

pub fn round(num: BigRational) -> BigRational {
    num.round()
}

pub fn pow(base: BigRational, exponent: BigRational) {
    todo!();
}

pub fn log(base: BigRational, numerus: BigRational) {
    todo!();
}

pub fn min(nums: Vec<BigRational>) -> BigRational {
    nums.iter().min().unwrap().to_owned()
}

pub fn max(nums: Vec<BigRational>) -> BigRational {
    nums.iter().max().unwrap().to_owned()
}

pub fn modulo(num1: BigRational, num2: BigRational) -> BigRational {
    num1%num2
}
