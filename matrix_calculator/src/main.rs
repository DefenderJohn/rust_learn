use std::time::Instant;

mod matrix;
use matrix::Matrix;

fn main() {
    let _word_size = 8;
    let _precision = 5;
    let _file_name = "result.txt".to_string();
    //let _matrix_a = Matrix::random_f64_matrix(20, 7, 12, 0.0, 1000.0);
    let matrix_b:Matrix<f64> = matrix::random_f64_matrix(10, 10, 28, 0.0, 1.0);
    let _scalar = 22;
    let start = Instant::now(); 

    let result = Matrix::inverse(&matrix_b);

    let duration = start.elapsed();
    Matrix::print_matrix(&matrix_b);
    //let _ = Matrix::write_to_file(&matrix_b, &file_name, word_size, precision);
    print!("求逆：\n");
    //print!("{}", _scalar);
    //Matrix::print_matrix(&matrix_b);
    print!("=\n");
    Matrix::print_matrix(&result);
    //let _ = Matrix::write_to_file(&result, &file_name, word_size, precision);
    //println!("{}", result);
    println!("Time elapsed: {} ns = {:?}", duration.as_nanos(), duration); 
}