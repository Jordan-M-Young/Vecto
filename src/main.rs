use crate::matrix::Matrix;

pub mod equation;
pub mod error;
pub mod matrix;
pub mod operations;
pub mod util;
fn main() {
    println!("Hello, world!");
    let vec_1 = vec![1.0, 1.0, 1.0];
    let vec_2 = vec![1.0, 1.0, 1.0];

    let vec3 = vec![1.0, 1.0];
    let vec4 = vec![1.0, 1.0];
    let vec5 = vec![1.0, 1.0];

    // let res1 = operations::add_vec(&vec_1, &vec_2);
    // let res2 = operations::sub_vec(&vec_1, &vec_2);
    // let res3 = operations::sum(&vec_1);
    // let res4 = operations::mean(&vec_1);

    // let res5 = operations::dot_product(&vec_1, &vec_2);
    // let res6 = operations::cosine_similarity(&vec_1, &vec_2);
    // let res7 = operations::scalar_multiply(&vec_1, 3.0);

    // let m7 = matrix::multiply_matrices(m2, m);
    let rows_1: Vec<Vec<f64>> = vec![
        vec![1.0, 4.0, 3.0, 3.0],
        vec![3.0, 1.0, 0.0, 4.0],
        vec![0.0, 0.0, 5.0, 5.0],
    ];
    let matrix = Matrix::new(rows_1).unwrap();

    let solved = equation::solve_system(matrix);

    println!("{:?}", solved);

    // println!("{:?}", m7)
}
