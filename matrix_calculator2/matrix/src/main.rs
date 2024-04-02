mod matrix;
mod complex;
mod equation;
mod test;

use bigdecimal::BigDecimal;
use complex::Complex;
fn main() {
    print!("{}", Complex::calc_sin(&BigDecimal::from(1)));
}
