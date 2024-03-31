use std::ops::{Add, Div, Mul, Sub};
use std::ptr::read;
use std::str::FromStr;
use bigdecimal::BigDecimal;
use num_traits::{FromPrimitive, Signed, ToPrimitive};

pub struct Complex {
    real:BigDecimal,
    imaginary:BigDecimal,
}

impl Complex {
    pub fn new() -> Complex{
        Complex{
            real: BigDecimal::from_f64(0.0).unwrap(),
            imaginary: BigDecimal::from_f64(0.0).unwrap()
        }
    }

    pub fn init(_real:f64, _imaginary:f64) -> Complex{
        Complex{
            real: BigDecimal::from_f64(_real).unwrap(),
            imaginary: BigDecimal::from_f64(_imaginary).unwrap()
        }
    }

    pub fn conjugate(target:&Complex) -> Complex{
        let mut result:Complex = Complex::new();
        result.real = target.real.clone();
        result.imaginary = -&target.imaginary;
        return result;
    }

    pub fn pow(target:&Complex, pow:i32) -> Complex{
        let (mut r, mut theta) = Self::to_polar(target);
        r = Self::fast_pow(&r, pow as u64);
        theta = theta * pow;
        return Self::from_polar(&r, &theta);
    }

    pub fn to_polar(target:&Complex) -> (BigDecimal, BigDecimal){
        let r = BigDecimal::sqrt(&(&BigDecimal::square(&target.real) + &BigDecimal::square(&target.imaginary))).unwrap();
        let theta = Self::calc_arctan(&(&target.imaginary / &target.real));
        return (r, theta);
    }

    pub fn from_polar(r:&BigDecimal, theta:&BigDecimal) -> Complex{
        let mut result:Complex = Complex::new();
        result.real = r * &Self::calc_cos(theta);
        result.imaginary = r * &Self::calc_sin(theta);
        return result;
    }

    fn calc_cos(angle:&BigDecimal) -> BigDecimal{
        let normalized_angle = Self::normalize_angle(&angle);
        let terms = 40;
        let mut result = BigDecimal::from(0);
        for term in 0..terms {
            let n = 2 * term;
            if term % 2 == 0 {
                let mut factorial:i128 = 1;
                for fact in 1..(n+1) as i128 {
                    factorial *= fact;
                }
                result -= Self::fast_pow(&angle, n) / factorial;
            }else {
                let mut factorial:i128 = 1;
                for fact in 1..(n+1) as i128 {
                    factorial *= fact;
                }
                result += Self::fast_pow(&angle, n) / factorial;
            }
        }
        return result;
    }

    fn calc_sin(angle:&BigDecimal) -> BigDecimal{
        let normalized_angle = Self::normalize_angle(&angle);
        let terms = 40;
        let mut result = BigDecimal::from(0);
        for term in 0..terms+1 {
            let n = 2 * term - 1;
            if term % 2 == 0 {
                let mut factorial:i128 = 1;
                for fact in 1..(n+1) as i128 {
                    factorial *= fact;
                }
                result -= Self::fast_pow(&angle, n) / factorial;
            }else {
                let mut factorial:i128 = 1;
                for fact in 1..(n+1) as i128 {
                    factorial *= fact;
                }
                result += Self::fast_pow(&angle, n) / factorial;
            }
        }
        return result;
    }

    fn normalize_angle(angle:&BigDecimal) -> BigDecimal{
        let PI = BigDecimal::from_str("3.141592653589793238462643383279").unwrap();
        let ratio:i64 = (&angle / (2 * &PI)).to_i64();
        let normalized_angle:BigDecimal = &angle - BigDecimal::from_i64(ratio).unwrap() * (2 * &PI);
        return normalized_angle;
    }

    fn calc_arctan(num:&BigDecimal) -> BigDecimal{
        let PI = BigDecimal::from_str("3.141592653589793238462643383279").unwrap();
        let terms = 10000;
        let mut result = BigDecimal::from(0);
        if BigDecimal::abs(&num) < BigDecimal::from(1) {
            for term in 1..terms {
                let n = 2 * term - 1;
                if term % 2 == 0 {
                    result -= Self::fast_pow(&num, n) / n;
                }else {
                    result += Self::fast_pow(&num, n) / n;
                }
            }
        }else if BigDecimal::eq(&num, &BigDecimal::from(1)) {
            result = &PI / 4;
            return result;
        }else if BigDecimal::eq(&num, &BigDecimal::from(-1)){
            result = -&PI / 4;
            return result;
        }else if BigDecimal::is_negative(&num) {
            result = -&PI / 2 - Self::calc_arctan(1 / num);
        }else if BigDecimal::is_positive(&num) {
            result = &PI / 2 - Self::calc_arctan(1 / num);
        }
        return result;
    }

    fn fast_pow(base: &BigDecimal, mut exponent: u64) -> BigDecimal {
        let mut result = BigDecimal::from_str("1").unwrap();
        let mut current_base = base.clone();

        while exponent > 0 {
            if exponent % 2 == 1 {
                result = &result * &current_base;
            }
            current_base = &current_base * &current_base;
            exponent /= 2;
        }

        return result;
    }
}

impl Add for &Complex{
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result:Complex = Complex::new();
        result.real = &self.real + &rhs.real;
        result.imaginary = &self.imaginary + &rhs.imaginary;
        return result;
    }
}

impl Sub for &Complex{
    type Output = Complex;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result:Complex = Complex::new();
        result.real = &self.real - &rhs.real;
        result.imaginary = &self.imaginary - &rhs.imaginary;
        return result;
    }
}

impl Mul for &Complex{
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result:Complex = Complex::new();
        result.real = &self.real * &rhs.real - &self.imaginary * &rhs.imaginary;
        result.imaginary = &self.real * &rhs.imaginary + &self.imaginary * &rhs.real;
        return result;
    }
}

impl Div for &Complex{
    type Output = Complex;

    fn div(self, rhs: Self) -> Self::Output {
        let mut result:Complex = Complex::new();
        let denominator = BigDecimal::square(&rhs.real) + BigDecimal::square(&rhs.imaginary);
        result.real = (&self.real * &rhs.real + &self.imaginary * &rhs.imaginary) / &denominator;
        result.imaginary = (&self.imaginary * &rhs.real - &self.real * &rhs.imaginary) / &denominator;
        return result;
    }
}