mod matrix;
mod complex;
mod equation;
mod test;

use std::str::FromStr;
use bigdecimal::BigDecimal;
use complex::Complex;
use crate::equation::EquationWithDegrees;

fn main() {
    let parameters = vec![
        Complex::from_big_decimal(&BigDecimal::from(-1), &BigDecimal::from(0)), // 常数项 -1
        Complex::from_big_decimal(&BigDecimal::from(-2), &BigDecimal::from(0)),  // x 的系数 2
        Complex::from_big_decimal(&BigDecimal::from(6), &BigDecimal::from(0)),  // x^2 的系数 3
        Complex::from_big_decimal(&BigDecimal::from(2), &BigDecimal::from(0)),  // x^3 的系数 2
    ];
    let equation = EquationWithDegrees::new(parameters);

    // 设置误差阈值
    let threshold = BigDecimal::from_str("0.0001").unwrap();

    // 求解
    let roots = equation.solve(&threshold);

    // 打印结果
    println!("2x³+5x²-2x-1=0 的解是:");
    for root_index in 0..roots.len() {
        let root = roots.get(root_index).unwrap();
        println!("x_{} = {} + {}i", root_index+1, root.real.round(10), root.imaginary.round(10));
    }
}
