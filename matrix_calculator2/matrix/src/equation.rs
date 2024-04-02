use std::str::FromStr;
use bigdecimal::BigDecimal;
use crate::complex::Complex;

pub struct EquationWithDegrees {
    parameters: Vec<Complex>,
}

impl EquationWithDegrees{
    pub fn new(parameters: Vec<Complex>) -> EquationWithDegrees{
        EquationWithDegrees{
            parameters,
        }
    }
    
    pub fn solve(&self, threshold:BigDecimal) -> Vec<Complex>{
        let ini_roots = Self::initial_roots(self.parameters.len());
        loop {
            let mut under_threshold = false;
            
            
            
            if under_threshold { 
                break;
            }
        }
        todo!()
    }
    
    fn initial_roots(size:usize) -> Vec<Complex>{
        let PI = BigDecimal::from_str("3.141592653589793238462643383279").unwrap();
        let mut result:Vec<Complex> = Vec::new();
        for i in 1..size {
            let real = Complex::calc_cos(&((2 * i as i64 * &PI) / size as i64), 5);
            let imaginary = Complex::calc_sin(&((2 * i as i64 * &PI) / size as i64), 5);
            let comp = Complex::from_big_decimal(&real, &imaginary);
            result.push(comp);
        }
        return result;
    }
    
    fn calc_eq(eq:EquationWithDegrees, x:Complex) -> Complex{
        let mut result = Complex::init(0.0, 0.0);
        for pow in eq.parameters.len() as i32..0 {
            let parameter = eq.parameters.get(pow as usize).unwrap();
            result = &result + &(parameter * &Complex::complex_pow(&x, pow));
        }
        return result;
    }
    
    fn calc_root_prods(roots:Vec<Complex>, current_root:Complex) -> Complex{
        let mut result = Complex::init(1.0, 0.0);
        for root in roots {
            if root.real != current_root.real || root.imaginary != current_root.imaginary {
                result = &result * &(&current_root - &root);
            }
        }
        return result;
    }
}