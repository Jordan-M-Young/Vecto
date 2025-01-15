use matrix::Matrix;
use models::cluster::kmeans;

pub mod algebra;
pub mod data;
pub mod error;
pub mod matrix;
pub mod models;
pub mod polynomial;
pub mod vector;

fn main() {
    // let rows = vec![
    //     vec![0.0, 0.0],
    //     vec![0.0, 0.5],
    //     vec![0.5, 0.0],
    //     vec![4.0, 4.0],
    //     vec![4.5, 4.0],
    //     vec![4.0, 5.0],
    // ];

    // let matrix = Matrix::new(rows).unwrap();

    // let cluster_res = kmeans(&matrix, 2).unwrap();

    // println!("{:?}", cluster_res)
    // let m = Matrix::new(rows).unwrap();
    // let n_centroids = 3;
    // let centroids = models::cluster::kpp_init(&m, n_centroids).unwrap();

    // let _targ_len: usize = 3;
    // println!("{:?}", centroids);

    // let res1 = operations::add_vec(&vec_1, &vec_2);
    // let res2 = operations::sub_vec(&vec_1, &vec_2);
    // let res3 = operations::sum(&vec_1);
    // let res4 = operations::mean(&vec_1);

    // let res5 = operations::dot_product(&vec_1, &vec_2);
    // let res6 = operations::cosine_similarity(&vec_1, &vec_2);
    // let res7 = operations::scalar_multiply(&vec_1, 3.0);

    // let m7 = matrix::multiply_matrices(m2, m);

    // println!("{:?}", m7)
    // let rows: Vec<Vec<f64>> = vec![
    //     vec![0.0],
    //     vec![0.8164965809277261],
    //     vec![-0.4082482904638631],
    //     vec![0.4082482904638631],
    // ];

    // let v: Matrix<f64> = Matrix::new(rows).unwrap();
    // let vt: Matrix<f64> = v.transpose();

    // let mut v_product = multiply_matrices(v, vt).unwrap();
    // v_product = scalar_multiply(&v_product, 2.0).unwrap();
    // let identity: Matrix<f64> = create_identity_matrix(4).unwrap();

    // let house_holder = sub_matrices(identity, v_product);
    // println!("{:?}", house_holder);

    // let rows_1: Vec<Vec<f64>> = vec![
    //     vec![4.0, 1.0, -2.0, 2.0],
    //     vec![1.0, 2.0, 0.0, 1.0],
    //     vec![-2.0, 0.0, 3.0, -2.0],
    //     vec![2.0, 1.0, -2.0, -1.0],
    // ];
    // let matrix = Matrix::new(rows_1).unwrap();
    // let m1 = tridiagonalize(&matrix);
    // println!("{:?}", m1);

    // let m =  Matrix {
    //     rows: vec![
    //         vec![4.0, -3.000000000000001, 1.332267629550197e-16, -9.325873406851313e-16],
    //         vec![-3.000000000000001, 3.3333333333333357, -1.666666666666667, -2.220446049250313e-16],
    //         vec![1.332267629550197e-16, -1.6666666666666665, -1.3200000000000016, 0.9066666666666631],
    //         vec![-9.325873406851313e-16, 0.0, 0.9066666666666627, 1.986666666666669]],
    //     m: 4,
    //     n: 4 };

    // println!("{:?}",is_tridiagonal(&m));

    // let m = tridiagonalize(&matrix);
    // let x: Vec<Vec<f64>> = vec![vec![1.0, 1.0], vec![1.0, 2.0], vec![1.0, 3.0]];
    // let y: Vec<Vec<f64>> = vec![vec![1.0], vec![2.0], vec![3.0]];

    // let features = Matrix::new(x).unwrap();
    // let targets = Matrix::new(y).unwrap();

    // let betas = models::linear_regression(&features, &targets);

    // println!("{:?}", betas)
}

// let determinant = matrix::operations::get_determinant(&m);

// println!("DETERMINANT: {:?}", determinant);
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
// let expanded = get_expanded_eigen_polynomial(&m);
// println!("{:?}", expanded)
// let inverse = cramer_inverse(&m);

// println!("{:?}", inverse);

// let trace = m.trace();
// println!("{}",trace)

// let equation = "2x^2 + 3x - 2".to_string();

// let poly = Polynomial::new(equation);
// let roots = poly.get_roots();

// println!("{:?}", poly);
// println!("{:?}", roots);
