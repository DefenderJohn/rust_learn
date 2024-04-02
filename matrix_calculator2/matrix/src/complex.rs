use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::ptr::read;
use std::str::FromStr;
use bigdecimal::BigDecimal;
use num_traits::{FromPrimitive, Signed, ToPrimitive};

#[derive(Clone)]
pub struct Complex {
    pub(crate) real:BigDecimal,
    pub(crate) imaginary:BigDecimal,
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

    pub fn from_big_decimal(_real:&BigDecimal, _imaginary:&BigDecimal) -> Complex{
        Complex{
            real: _real.clone(),
            imaginary: _imaginary.clone()
        }
    }

    pub fn conjugate(target:&Complex) -> Complex{
        let mut result:Complex = Complex::new();
        result.real = target.real.clone();
        result.imaginary = -&target.imaginary;
        return result;
    }
    
    pub fn complex_pow(target:&Complex, pow:i32) -> Complex{
        let mut result = Complex::init(1.0, 0.0);
        for _ in 0..pow {
            result = &result * target;
        }
        return result;
    }

    pub fn pow(target:&Complex, pow:i32) -> Complex{
        let (mut r, mut theta) = Self::to_polar(target);
        r = Self::fast_pow(&r, pow as u64);
        theta = theta * pow;
        print!("{}\n",Self::from_big_decimal(&r,&theta));
        return Self::from_polar(&r, &theta);
    }

    pub fn to_polar(target:&Complex) -> (BigDecimal, BigDecimal){
        let r = BigDecimal::sqrt(&(&BigDecimal::square(&target.real) + &BigDecimal::square(&target.imaginary))).unwrap();
        let theta = Self::calc_arctan(&(&target.imaginary / &target.real));
        return (r, theta);
    }

    pub fn from_polar(r:&BigDecimal, theta:&BigDecimal) -> Complex{
        let mut result:Complex = Complex::new();
        result.real = r * &Self::calc_cos(theta, 10);
        result.imaginary = r * &Self::calc_sin(theta, 10);
        print!("{}\n",&result);
        return result;
    }

    pub fn calc_cos(angle: &BigDecimal, terms:u64) -> BigDecimal {
        let normalized_angle = Self::normalize_angle(&angle);
        let mut result = BigDecimal::from(0);
        let mut factorial = BigDecimal::from(1);
        for term in 0..terms {
            let n = 2 * term;
            if term != 0 {
                // 对于n > 0，计算n! = n * (n-1)!
                factorial *= BigDecimal::from(n * (n - 1));
            }
            if term % 2 == 1 {
                result -= Self::fast_pow(&normalized_angle, n) / &factorial;
            } else {
                result += Self::fast_pow(&normalized_angle, n) / &factorial;
            }
        }
        return result;
    }

    pub fn calc_sin(angle: &BigDecimal, terms:u64) -> BigDecimal {
        let normalized_angle = Self::normalize_angle(&angle);
        let terms = 30;
        let mut result = BigDecimal::from(0);
        let mut factorial = BigDecimal::from(1);
        for term in 0..terms {
            let n = 2 * term + 1;
            if term != 0 {
                // 更新阶乘值以包含下一个奇数项的阶乘
                factorial *= BigDecimal::from((2 * term) * (2 * term + 1));
            }
            if term % 2 == 1 {
                result -= Self::fast_pow(&normalized_angle, n) / &factorial;
            } else {
                result += Self::fast_pow(&normalized_angle, n) / &factorial;
            }
        }
        return result;
    }


    fn normalize_angle(angle:&BigDecimal) -> BigDecimal{
        let PI = BigDecimal::from_str("3.141592653589793238462643383279").unwrap();
        let ratio = (angle / &(2 * &PI)).to_i64().unwrap();
        let normalized_angle:BigDecimal = angle - BigDecimal::from_i64(ratio).unwrap() * (2 * &PI);
        return normalized_angle;
    }

    fn calc_arctan(mut num:&BigDecimal) -> BigDecimal{
        let num = num.round(40);
        let PI = BigDecimal::from_str("3.141592653589793238462643383279").unwrap();
        let terms = 1000;
        let mut result = BigDecimal::from(0);
        if BigDecimal::abs(&num) < BigDecimal::from(1) {
            for term in 1..terms {
                let n = 2 * term - 1;
                if term % 2 == 0 {
                    result -= Self::fast_pow(&num, n) / n;
                }else {
                    result += Self::fast_pow(&num, n) / n;
                }
                result = result.round(50);
            }
        }else if BigDecimal::eq(&num, &BigDecimal::from(1)) {
            result = &PI / 4;
            return result;
        }else if BigDecimal::eq(&num, &BigDecimal::from(-1)){
            result = -&PI / 4;
            return result;
        }else if BigDecimal::is_negative(&num) {
            result = -&PI / 2 - Self::calc_arctan(&(&BigDecimal::from(1) / num));
        }else if BigDecimal::is_positive(&num) {
            result = &PI / 2 - Self::calc_arctan(&(&BigDecimal::from(1) / num));
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
    
    pub fn exp(target:&Complex) -> Complex{
        let mut result = Complex::new();
        let real_exp = target.real.exp();
        result.real = real_exp.clone() * Self::calc_cos(&target.imaginary, 5);
        result.imaginary = real_exp.clone() * Self::calc_sin(&target.imaginary, 5);
        return result;
    }
    
    pub fn abs(&self) -> BigDecimal{
        return BigDecimal::sqrt(&(&self.real * &self.real + &self.imaginary * &self.imaginary)).unwrap()  
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rounded_real = self.real.with_scale(10);
        let real: f64 = rounded_real.to_f64().unwrap();
        let rounded_imaginary = self.imaginary.with_scale(10);
        let imaginary: f64 = rounded_imaginary.to_f64().unwrap();
        write!(f, "{} + {}i", real, imaginary)
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