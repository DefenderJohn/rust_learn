use std::str::FromStr;
use bigdecimal::BigDecimal;
use num_traits::ToPrimitive;
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

    pub fn solve(&self, threshold:&BigDecimal) -> Vec<Complex>{
        let mut roots = Self::initial_roots(self.parameters.len() - 1);
        let mut temp_roots = Vec::new();
        loop {
            let mut errors:Vec<BigDecimal> = Vec::new();
            for root in &roots {
                let res = Self::calc_eq(self, root);
                res.real.round(50);
                res.imaginary.round(50);
                let error = res.abs();
                errors.push(error);
                let root_prod = Self::calc_root_prods(&roots, root);
                root_prod.real.round(50);
                root_prod.imaginary.round(50);
                let new_root = root - &(&res / &root_prod);
                new_root.real.round(20);
                new_root.imaginary.round(20);
                temp_roots.push(new_root);
            }
            roots = temp_roots.clone();
            temp_roots = Vec::new();
            let mut under_threshold = true;
            print!("Errors:");
            for error_index in 0..errors.len() {
                let error = errors.get(error_index).unwrap();
                let print_error = error.clone();
                print_error.round(4);
                print!("{} ",print_error.to_f64().unwrap());
                if error.abs() > *threshold { 
                    under_threshold = false;
                }
            }
            errors = Vec::new();
            print!("\n");
            if under_threshold { 
                break;
            }
        }
        return roots;
    }

    fn initial_roots(size:usize) -> Vec<Complex>{
        let PI = BigDecimal::from_str("3.141592653589793238462643383279").unwrap();
        let mut result:Vec<Complex> = Vec::new();
        for i in 1..=size {
            let real =  Complex::calc_cos(&((2 * i as i64 * &PI) / size as i64), 4).round(10);
            let imaginary = Complex::calc_sin(&((2 * i as i64 * &PI) / size as i64), 4).round(10);
            let comp = Complex::from_big_decimal(&real, &imaginary);
            result.push(comp);
        }
        return result;
    }

    fn calc_eq(eq:&EquationWithDegrees, x:&Complex) -> Complex{
        let mut result = Complex::from_big_decimal(&eq.parameters.get(0).unwrap().real, &eq.parameters.get(0).unwrap().imaginary);
        for pow in 1 .. eq.parameters.len() as i32 {
            let parameter = eq.parameters.get(pow as usize).unwrap();
            result = &result + &(parameter * &Complex::complex_pow(&x, pow));
        }
        return result;
    }

    fn calc_root_prods(roots:&Vec<Complex>, current_root:&Complex) -> Complex{
        let mut result = Complex::init(1.0, 0.0);
        for root in roots {
            root.real.round(50);
            root.imaginary.round(50);
            if root.real != current_root.real || root.imaginary != current_root.imaginary {
                result = &result * &(current_root - root);
            }
        }
        return result;
    }
}