use bigdecimal::{BigDecimal, FromPrimitive};
use std::str::FromStr;
use std::ops::{Add, Sub, Mul, Div};

#[cfg(test)]
mod tests {
    use crate::complex::Complex;
    use super::*;

    #[test]
    fn test_new() {
        let complex = Complex::new();
        assert_eq!(complex.real, BigDecimal::from_f64(0.0).unwrap());
        assert_eq!(complex.imaginary, BigDecimal::from_f64(0.0).unwrap());
    }

    #[test]
    fn test_init() {
        let complex = Complex::init(3.0, 4.0);
        assert_eq!(complex.real, BigDecimal::from_f64(3.0).unwrap());
        assert_eq!(complex.imaginary, BigDecimal::from_f64(4.0).unwrap());
    }

    #[test]
    fn test_conjugate() {
        let complex = Complex::init(3.0, 4.0);
        let conjugate = Complex::conjugate(&complex);
        assert_eq!(conjugate.real, BigDecimal::from_f64(3.0).unwrap());
        assert_eq!(conjugate.imaginary, BigDecimal::from_f64(-4.0).unwrap());
    }

    #[test]
    fn test_add() {
        let complex1 = Complex::init(3.0, 4.0);
        let complex2 = Complex::init(1.0, 2.0);
        let result = &complex1 + &complex2;
        assert_eq!(result.real, BigDecimal::from_f64(4.0).unwrap());
        assert_eq!(result.imaginary, BigDecimal::from_f64(6.0).unwrap());
    }

    #[test]
    fn test_sub() {
        let complex1 = Complex::init(5.0, 7.0);
        let complex2 = Complex::init(3.0, 2.0);
        let result = &complex1 - &complex2;
        assert_eq!(result.real, BigDecimal::from_f64(2.0).unwrap());
        assert_eq!(result.imaginary, BigDecimal::from_f64(5.0).unwrap());
    }

    #[test]
    fn test_mul() {
        let complex1 = Complex::init(1.0, 2.0);
        let complex2 = Complex::init(3.0, 4.0);
        let result = &complex1 * &complex2;
        // (1 + 2i) * (3 + 4i) = -5 + 10i
        assert_eq!(result.real, BigDecimal::from_f64(-5.0).unwrap());
        assert_eq!(result.imaginary, BigDecimal::from_f64(10.0).unwrap());
    }

    #[test]
    fn test_div() {
        let complex1 = Complex::init(3.0, 2.0);
        let complex2 = Complex::init(4.0, -3.0);
        let result = &complex1 / &complex2;
        // (3 + 2i) / (4 - 3i) = 6/25 + 17/25i
        assert!(result.real - BigDecimal::from_f64(6.0/25.0).unwrap() < BigDecimal::from_f64(0.0001).unwrap());
        assert!(result.imaginary - BigDecimal::from_f64(17.0/25.0).unwrap() < BigDecimal::from_f64(0.0001).unwrap());
    }

    #[test]
    fn test_pow() {
        let complex = Complex::init(2.0, 3.0);
        let result = Complex::complex_pow(&complex, 5);
        // (1 + i)^2 = 1^2 + 2*1*i + i^2 = 2i
        print!("{}",result);
        assert!((BigDecimal::abs(&(result.real - BigDecimal::from_f64(122.0).unwrap()))) < BigDecimal::from_f64(0.0001).unwrap());
        assert!((BigDecimal::abs(&(result.imaginary - BigDecimal::from_f64(-597.0).unwrap()))) < BigDecimal::from_f64(0.0001).unwrap());

    }

    #[test]
    fn test_to_polar_and_from_polar() {
        let complex = Complex::init(1.0, 1.0);
        let (r, theta) = Complex::to_polar(&complex);
        // r = sqrt(1^2 + 1^2) = sqrt(2), theta = arctan(1/1) = pi/4
        let expected_r = BigDecimal::sqrt(&BigDecimal::from_f64(2.0).unwrap()).unwrap();
        let expected_theta = BigDecimal::from_f64(std::f64::consts::PI / 4.0).unwrap();
        assert!(&r - expected_r < BigDecimal::from_f64(0.0001).unwrap());
        assert!(&theta - expected_theta < BigDecimal::from_f64(0.0001).unwrap());

        let complex_back = Complex::from_polar(&r, &theta);
        assert!(complex_back.real - BigDecimal::from_f64(1.0).unwrap() < BigDecimal::from_f64(0.0001).unwrap());
        assert!(complex_back.imaginary - BigDecimal::from_f64(1.0).unwrap() < BigDecimal::from_f64(0.0001).unwrap());
    }
}

// 确保将测试放在正确的模块内，并且对于那些依赖具体实现的测试，你需要根据你的方法实现来具体编写测试用例。
