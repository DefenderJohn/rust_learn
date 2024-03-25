use std::fs::{File, OpenOptions};
use std::ops::{Add, Div, Mul, Sub};
use std::time::Instant;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use num_traits::One;
use std::io::Write;

#[derive(Clone)]
struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> where T: 
      Copy
    + Default
    + One 
    + Add<Output = T>
    + Div<Output = T>
    + Mul<Output = T>
    + Sub<Output = T>
    + std::fmt::Display
    + PartialOrd,  {
    fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }

    fn init(rows:usize, cols: usize, data:Vec<T>) -> Self {
        let mut result: Matrix<T> = Matrix::new(rows, cols);
        result.data = data;
        return result;
    }

    fn print_matrix(matrix: &Matrix<T>){
        let width = 6;
        print!("┌");
        print!("{}", " ".repeat((width+1) * matrix.get_cols()));
        print!("┐");
        println!();
        for row in 0..matrix.get_rows() {
            print!("│");
            for col in 0..matrix.get_cols() {
                let element: String = format!("{:.3},", matrix.get(row, col));
                print!("{:^width$} ", element, width = width);
            }
            print!("│");
            println!();
        }
        print!("└");
        print!("{}", " ".repeat((width+1) * matrix.get_cols()));
        print!("┘");
        println!();
    }

    fn write_to_file(matrix: &Matrix<T>, file_name: &String, word_size:usize, decimal_space:usize) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_name)?;
    
        let width = word_size;
        write!(file, "┌")?;
        write!(file, "{}", " ".repeat((width+1) * matrix.get_cols()))?;
        write!(file, "┐\n")?;

        for row in 0..matrix.get_rows() {
            write!(file, "│")?;
            for col in 0..matrix.get_cols() {
                let element = format!("{:.precision$},", matrix.get(row, col).to_string(),precision=decimal_space);
                write!(file, "{:^width$} ", element, width = width)?;
            }
            write!(file, "│\n")?;
        }

        write!(file, "└")?;
        write!(file, "{}", " ".repeat((width+1) * matrix.get_cols()))?;
        write!(file, "┘\n")?;

        Ok(())
    }

    fn set(&mut self, row: usize, col: usize, value: T) {
        assert!(row < self.rows && col < self.cols, "Index out of bounds.");
        let index: usize = self.cols * row + col;
        self.data[index] = value;
    }

    fn get(&self, row: usize, col: usize) -> &T {
        assert!(row < self.rows && col < self.cols, "Index out of bounds.");
        let index: usize = self.cols * row + col;
        &self.data[index]
    }

    fn get_rows(&self) -> usize{
        return self.rows;
    }

    fn get_cols(&self) -> usize{
        return self.cols;
    }

    fn transpose(target:&Matrix<T>) -> Matrix<T>{
        let mut result: Matrix<T> = Matrix::new(target.get_cols(),target.get_rows());
        for row in 0..target.get_cols() {
            for col in 0..target.get_rows() {
                result.set(row, col, *target.get(col, row));
            }
        }
        return result;
    }

    fn determinant(target:&Matrix<T>) -> T{
        let (upper_triangular_matrix,swap_ops) = Matrix::get_upper_triangular_matrix(target);
        let mut result: T = *upper_triangular_matrix.get(0, 0);
        for index in 1..upper_triangular_matrix.get_cols().min(upper_triangular_matrix.get_rows()) {
            result = result * *upper_triangular_matrix.get(index, index);
        }
        if swap_ops % 2 == 0 {
            return result;
        }else{
            return T::default() - result;
        }
    }

    fn get_upper_triangular_matrix(target:&Matrix<T>) -> (Matrix<T>, i64){
        let mut result_matrix: Matrix<T> = Matrix::init(target.get_rows(), target.get_cols(), target.data.clone());
        let mut swap_ops: i64 = 0;
        for col_index in 0 .. result_matrix.get_cols(){
            let mut pivot : T = *result_matrix.get(col_index, col_index);
            let mut pivot_index : usize = col_index;
            for row_index in col_index + 1 .. result_matrix.get_rows(){
                let item = *result_matrix.get(row_index, col_index);
                if item > pivot || T::default() - item > pivot {
                   pivot = item; 
                   pivot_index = row_index;
                }
            }

            if pivot_index != col_index{
                swap_ops += 1;
                result_matrix.swap_rows(pivot_index, col_index);
            }

            for row_index in col_index + 1 .. result_matrix.get_rows() {
                let first_item: T = *result_matrix.get(row_index, col_index);
                let ratio: <T as Sub<T>>::Output = T::default() - (first_item / pivot);
                for sub_col_index in col_index..result_matrix.get_cols() {
                    result_matrix.set(row_index, sub_col_index, 
                        *result_matrix.get(row_index, sub_col_index) + 
                        ratio * *result_matrix.get(col_index, sub_col_index));
                }
            }
        }
        return (result_matrix, swap_ops);
    }

    fn swap_rows(&mut self, from_row: usize, to_row: usize){
        let mut cache:T;
        for col in 0 .. self.get_cols() {
            cache = *self.get(from_row, col);
            self.set(from_row, col, *self.get(to_row, col));
            self.set(to_row, col, cache);
        }
    }

    fn inverse(target:&Matrix<T>)-> Matrix<T>{
        let mut result_matrix = Matrix::get_matrix_of_minors(target);
        result_matrix = Matrix::get_matrix_of_cofactors(&result_matrix);
        result_matrix = Matrix::transpose(&result_matrix);
        result_matrix = &result_matrix * (T::one() / Matrix::determinant(&target));
        return result_matrix;
    }

    fn get_matrix_of_cofactors(target:&Matrix<T>) -> Matrix<T>{
        let mut result_matrix: Matrix<T> = Matrix::new(target.get_rows(),target.get_cols());
        for row in 0..target.get_rows() {
            for col in 0..target.get_cols(){
                if (row + col) % 2 == 0 {
                    result_matrix.set(row, col, *target.get(row, col));
                }else{
                    result_matrix.set(row, col, T::default() - *target.get(row, col));
                }
            }
        }
        return result_matrix;
    }

    fn get_matrix_of_minors(target:&Matrix<T>) -> Matrix<T>{
        let mut result_matrix: Matrix<T> = Matrix::new(target.get_rows(),target.get_cols());
        for selected_row in 0..target.get_rows(){
            for selected_col in 0..target.get_cols(){
                let mut temp_matrix : Matrix<T> = Matrix::new(target.get_rows()-1,target.get_cols()-1);
                let mut temp_row = 0;
                for i in 0..target.get_rows() {
                    if i == selected_row { 
                        continue; 
                    }
                    let mut temp_col: usize = 0;
                    for j in 0..target.get_cols() {
                        if j == selected_col { 
                            continue; 
                        }
                        temp_matrix.set(temp_row, temp_col, *target.get(i, j));
                        temp_col += 1;
                    }
                    temp_row += 1;
                }
                let determinant: T = Matrix::determinant(&temp_matrix);
                result_matrix.set(selected_row, selected_col, determinant);
            }
        }
        return result_matrix;
    }

    fn hadamard_product(lhs:Matrix<T>, rhs:Matrix<T>) -> Matrix<T>{
        assert!(lhs.get_cols() == rhs.get_cols() && lhs.get_rows() == rhs.get_rows(), "矩阵大小不匹配");
        let mut result_matrix: Matrix<T> = Matrix::new(lhs.get_rows(),lhs.get_cols());
        for index in 0..lhs.get_cols() * lhs.get_rows(){
            result_matrix.data[index] = lhs.data[index] * rhs.data[index];
        }
        return result_matrix;
    }
}

impl<T> Add for &Matrix<T> where 
        T: Copy + Default + PartialOrd + One + Add<Output = T> + Div<Output = T> + Mul<Output = T> + Sub<Output = T> + std::fmt::Display{
    type Output = Matrix<T>;

    fn add(self, other: Self) -> Self::Output {
        assert!(self.get_cols() == other.get_cols() && self.get_rows() == other.get_rows(), "矩阵大小不匹配");
        let mut result_matrix: Matrix<T> = Matrix::new(self.get_rows(),self.get_cols());
        for element_index in 0..self.get_cols() * self.get_rows() {
            result_matrix.data[element_index] = self.data[element_index] + other.data[element_index];
        }
        return result_matrix;
    }
}

impl<T> Sub for &Matrix<T> where 
        T: Copy + Default + PartialOrd + One + Add<Output = T> + Div<Output = T> + Mul<Output = T> + Sub<Output = T> + std::fmt::Display{
    type Output = Matrix<T>;

    fn sub(self, other: Self) -> Self::Output {
        assert!(self.get_cols() == other.get_cols() && self.get_rows() == other.get_rows(), "矩阵大小不匹配");
        let mut result_matrix: Matrix<T> = Matrix::new(self.get_rows(),self.get_cols());
        for element_index in 0..self.get_cols() * self.get_rows() {
            result_matrix.data[element_index] = self.data[element_index] - other.data[element_index];
        }
        return result_matrix;
    }
}

impl<T> Mul for &Matrix<T> where 
        T: Copy + Default + PartialOrd + One + Add<Output = T> + Div<Output = T> + Mul<Output = T> + Sub<Output = T> + std::fmt::Display{
    type Output = Matrix<T>;

    fn mul(self, other: Self) -> Self::Output {
        assert!(self.get_cols() == other.get_rows(), "矩阵大小不匹配");
        let mut result_matrix: Matrix<T> = Matrix::new(self.get_rows(),other.get_cols());
        for i in 0..self.rows{
            for j in 0..other.cols{
                let mut sum = T::default();
                for k in 0 .. self.get_cols(){
                    sum = sum + (*self.get(i, k)) * (*other.get(k, j));
                }
                result_matrix.set(i, j, sum);
            }
        }
        return result_matrix;
    }
}

impl<T> Mul<T> for &Matrix<T> where 
        T: Copy + Default + PartialOrd + One + Add<Output = T> + Div<Output = T> + Mul<Output = T> + Sub<Output = T> + std::fmt::Display {
    type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Self::Output {
        let mut result_matrix: Matrix<T> = Matrix::new(self.get_rows(),self.get_cols());
        for index in 0..self.get_cols() * self.get_rows(){
            result_matrix.data[index] = self.data[index] * rhs;
        }
        return result_matrix;
    }
}

fn random_i64_matrix(rows:usize, cols: usize, random_seed: i64, range_lower_bound: i64, range_upper_bound: i64) -> Matrix<i64>{
    let mut data:Vec<i64> = Vec::new();
    let seed_u64: u64 = random_seed as u64;
    let mut rng: StdRng = StdRng::seed_from_u64(seed_u64);
    for _ in 0 .. rows * cols {
        let random_num: i64 = rng.gen_range(range_lower_bound ..= range_upper_bound);
        data.push(random_num);
    }
    let mut result: Matrix<i64> = Matrix::new(rows, cols);
    result.data = data;
    return result;
}

fn random_f64_matrix(rows:usize, cols: usize, random_seed: i64, range_lower_bound: f64, range_upper_bound: f64) -> Matrix<f64>{
    let mut data:Vec<f64> = Vec::new();
    let seed_u64: u64 = random_seed as u64;
    let mut rng: StdRng = StdRng::seed_from_u64(seed_u64);
    for _ in 0 .. rows * cols {
        let random_num: f64 = rng.gen_range(range_lower_bound ..= range_upper_bound);
        data.push(random_num);
    }
    let mut result: Matrix<f64> = Matrix::new(rows, cols);
    result.data = data;
    return result;
}

fn main() {
    let word_size = 8;
    let precision = 5;
    let file_name = ("result.txt").to_string();
    let matrix_a = random_f64_matrix(20, 7, 12, 0.0, 1000.0);
    let matrix_b = random_f64_matrix(50, 50, 28, 0.0, 1.0);
    let scaler = 22;
    let start = Instant::now(); 

    let result = Matrix::inverse(&matrix_b);

    let duration = start.elapsed();

    let _ = Matrix::write_to_file(&matrix_b, &file_name, word_size, precision);
    //print!("求逆：\n");
    //print!("{}", scaler);
    //Matrix::print_matrix(&matrix_b);
    //print!("=\n");
    let _ = Matrix::write_to_file(&result, &file_name, word_size, precision);
    //println!("{}", result);
    println!("Time elapsed: {} ns = {:?}", duration.as_nanos(), duration); 
}