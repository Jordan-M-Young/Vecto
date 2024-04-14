use matrix::inverse::cramer_inverse;

use crate::{
    matrix::Matrix,
    polynomial::{parse_equation, Polynomial},
};

pub mod algebra;
pub mod error;
pub mod matrix;
pub mod polynomial;
pub mod vector;

fn main() {
    let _vec_1 = vec![1.0, 1.0, 1.0];
    let _vec_2 = vec![1.0, 1.0, 1.0];

    let _vec3 = vec![1.0, 1.0];
    let _vec4 = vec![1.0, 1.0];
    let _vec5 = vec![1.0, 1.0];

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

    let solved = algebra::solve_system(matrix);

    println!("{:?}", solved);

    let a: i32 = 8;
    let b = 2;

    let _c: i32 = a.rem_euclid(b);
    // println!("{:?}", m7)

    let rows = vec![
        vec![5, 1, -1, 0],
        vec![-2, 4, 2, -1],
        vec![3, 3, 3, 3],
        vec![1, 2, 3, 4],
    ];

    let m = Matrix::new(rows).unwrap();
    let determinant = matrix::operations::get_determinant(&m);

    println!("DETERMINANT: {:?}", determinant);
    // let size: usize = v.len();
    // let mut v = vec![];
    // let perms = util::heap_permutation(&mut v, size);

    // for mut perm in perms {
    //     println!("Perm {:?}",perm);
    //     let sign = get_permutation_sign(perm);
    //     println!("Sign {:?}",sign);

    // }

    // let rows = vec![vec![1, 1, 1], vec![3, 2, 1], vec![2, 1, 3]];

    // let m = Matrix::new(rows).unwrap();

    // let inverse = cramer_inverse(&m);

    // println!("{:?}", inverse);

    // let trace = m.trace();
    // println!("{}",trace)

    let equation = "3x - 2".to_string();

    let poly = Polynomial::new(equation);
    let roots = poly.get_roots();

    println!("{:?}", poly);
    println!("{:?}", roots);
}
