use crate::error::{self, CustomErrors, MismatchError};
use crate::matrix::logic::{can_add, can_multiply, is_square};
use crate::matrix::Matrix;
use crate::vector::operations::{add_vec, sub_vec};
use crate::vector::util;

pub fn add_matrices<T: std::marker::Copy + std::ops::Add<Output = T>>(
    matrix_1: Matrix<T>,
    matrix_2: Matrix<T>,
) -> Result<Matrix<T>, error::CustomErrors> {
    if !can_add(&matrix_1, &matrix_2) {
        return Err(error::CustomErrors::Mismatch(error::MismatchError));
    }

    let mut new_matrix_rows: Vec<Vec<T>> = vec![];
    for i in 0..matrix_1.m {
        let new_row = match add_vec(&matrix_1.rows[i], &matrix_2.rows[i]) {
            Ok(vec) => vec,
            Err(e) => return Err(e),
        };
        new_matrix_rows.push(new_row)
    }
    Ok(Matrix {
        rows: new_matrix_rows,
        m: matrix_1.m,
        n: matrix_1.n,
    })
}

pub fn sub_matrices<T: std::marker::Copy + std::ops::Sub<Output = T>>(
    matrix_1: Matrix<T>,
    matrix_2: Matrix<T>,
) -> Result<Matrix<T>, error::CustomErrors> {
    if !can_add(&matrix_1, &matrix_2) {
        return Err(error::CustomErrors::Mismatch(error::MismatchError));
    }

    let mut new_matrix_rows: Vec<Vec<T>> = vec![];
    for i in 0..matrix_1.m {
        let new_row = match sub_vec(&matrix_1.rows[i], &matrix_2.rows[i]) {
            Ok(vec) => vec,
            Err(e) => return Err(e),
        };
        new_matrix_rows.push(new_row)
    }
    Ok(Matrix {
        rows: new_matrix_rows,
        m: matrix_1.m,
        n: matrix_1.n,
    })
}

pub fn scalar_multiply<
    T: Copy
        + Into<f64>
        + std::ops::Mul
        + std::ops::Mul<Output = T>
        + std::convert::From<u8>
        + std::ops::AddAssign,
>(
    matrix: &Matrix<T>,
    scalar: T,
) -> Result<Matrix<T>, error::CustomErrors> {
    let rows = &matrix.rows;
    let m = matrix.m;
    let n = matrix.n;

    let mut new_rows: Vec<Vec<T>> = vec![];

    for i in 0..m {
        let mut new_row: Vec<T> = vec![];
        for j in 0..n {
            let new_val = rows[i][j] * scalar;
            new_row.push(new_val)
        }
        new_rows.push(new_row)
    }

    match Matrix::new(new_rows) {
        Ok(matrix) => Ok(matrix),
        Err(err) => Err(err),
    }
}

pub fn scalar_divide<T: Copy + Into<f64> + std::ops::Div + std::ops::Div<Output = T>>(
    matrix: &Matrix<T>,
    scalar: T,
) -> Result<Matrix<f64>, CustomErrors> {
    let rows = &matrix.rows;
    let m = matrix.m;
    let n = matrix.n;
    let scalar: f64 = scalar.into();

    let mut new_rows: Vec<Vec<f64>> = vec![];

    for i in 0..m {
        let mut new_row: Vec<f64> = vec![];
        for j in 0..n {
            let old_val: f64 = rows[i][j].into();
            let new_val = old_val / scalar;
            new_row.push(new_val)
        }
        new_rows.push(new_row)
    }

    match Matrix::new(new_rows) {
        Ok(matrix) => Ok(matrix),
        Err(err) => Err(err),
    }
}

pub fn multiply_matrices<
    T: Copy + From<u8> + std::ops::Mul + std::ops::AddAssign<<T as std::ops::Mul>::Output>,
>(
    matrix_1: &Matrix<T>,
    matrix_2: &Matrix<T>,
) -> Result<Matrix<T>, error::CustomErrors> {
    if matrix_1.m == 0 || matrix_1.n == 0 || matrix_2.m == 0 || matrix_2.n == 0 {
        return Err(error::CustomErrors::EmptyVector(error::EmptyVectorError));
    }

    if !can_multiply(&matrix_1, &matrix_2) {
        return Err(error::CustomErrors::Mismatch(error::MismatchError));
    }

    let mut new_matrix_rows: Vec<Vec<T>> = vec![];

    for i in 0..matrix_1.m {
        let mut new_row: Vec<T> = vec![];
        for j in 0..matrix_2.n {
            let mut element_sum: T = 0.into();
            for k in 0..matrix_1.n {
                element_sum += matrix_1.rows[i][k] * matrix_2.rows[k][j]
            }
            new_row.push(element_sum)
        }
        new_matrix_rows.push(new_row)
    }

    Ok(Matrix {
        rows: new_matrix_rows,
        m: matrix_1.m,
        n: matrix_2.n,
    })
}

pub fn get_determinant<
    T: From<u8>
        + From<i32>
        + Copy
        + std::ops::Mul<Output = T>
        + std::ops::AddAssign
        + std::fmt::Debug
        + std::ops::MulAssign,
>(
    matrix: &Matrix<T>,
) -> Result<T, error::CustomErrors> {
    if !is_square(matrix) {
        return Err(error::CustomErrors::NotImplemented(
            error::NotImplementedError,
        ));
    }

    let size = matrix.rows.len();
    let rows = &matrix.rows;
    let perms = util::get_perms(size);
    let zero_cast: T = 0.into();
    let one_cast: T = 1.into();
    let mut determinant: T = zero_cast;
    for perm in perms {
        let sign = util::get_permutation_sign(perm.clone());
        let sign: T = sign.into();
        let mut term: T = one_cast;
        for i in 0..size {
            term *= rows[i][perm[i]]
        }
        determinant += sign * term;
    }

    Ok(determinant)
}

pub fn multiply_matrix_vector(mat: &Matrix<f64>, vec: &Vec<f64>) -> Result<Vec<f64>, CustomErrors> {
    let m = mat.m;
    let n = mat.n;

    if n != vec.len() {
        return Err(CustomErrors::Mismatch(MismatchError));
    }

    let rows = &mat.rows;

    let mut prod: Vec<f64> = vec![];

    for i in 0..m {
        let mut row_prod = 0.0;
        for j in 0..n {
            let mat_item = rows[i][j];
            let vec_item = vec[j];
            row_prod += mat_item * vec_item
        }
        prod.push(row_prod)
    }

    Ok(prod)
}

//tests......
// ------------------------------
// ------------------------------

#[cfg(test)]
mod tests {
    use crate::{
        error::{CustomErrors, MismatchError},
        matrix::{operations, Matrix},
    };

    use super::{multiply_matrices, multiply_matrix_vector};
    #[test]
    fn test_multiply_matrix_vector() {
        let mat_rows = vec![vec![1.0, -1.0, 2.0], vec![0.0, -3.0, 1.0]];
        let mat = Matrix::new(mat_rows).unwrap();

        let vec = vec![2.0, 1.0, 0.0];

        let assumed = vec![1.0, -3.0];

        let target = multiply_matrix_vector(&mat, &vec).unwrap();

        assert_eq!(assumed, target);

        let vec2 = vec![2.0, 1.0];

        let assumed = CustomErrors::Mismatch(MismatchError);
        match multiply_matrix_vector(&mat, &vec2) {
            Ok(_) => {}
            Err(err) => {
                assert_eq!(err, assumed);
            }
        }
    }

    #[test]
    fn test_add_matrices_2_x_2() {
        let rows = vec![vec![5, -1], vec![-2, 4]];

        let rows_2 = rows.clone();

        let m = Matrix::new(rows).unwrap();
        let m2 = Matrix::new(rows_2).unwrap();
        let rows_3 = vec![vec![10, -2], vec![-4, 8]];
        let m3 = Matrix::new(rows_3).unwrap();
        match operations::add_matrices(m, m2) {
            Ok(val) => {
                assert_eq!(val, m3)
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_get_determinant_2_x_2() {
        let rows = vec![vec![5, -1], vec![-2, 4]];

        let m = Matrix::new(rows).unwrap();
        match operations::get_determinant(&m) {
            Ok(result) => {
                assert_eq!(result, 18)
            }
            _ => assert!(false),
        };
    }

    #[test]
    fn test_multiply_matrices() {
        let vec_1 = vec![1.0, 1.0, 1.0];
        let vec_2 = vec![1.0, 1.0, 1.0];

        let vec3 = vec![1.0, 1.0];
        let vec4 = vec![1.0, 1.0];
        let vec5 = vec![1.0, 1.0];

        let m = Matrix::new(vec![vec_1, vec_2]).unwrap();
        let m2 = Matrix::new(vec![vec3, vec4, vec5]).unwrap();

        let m3: Matrix<f64> = Matrix::new(vec![vec![3.0, 3.0], vec![3.0, 3.0]]).unwrap();
        match multiply_matrices(&m, &m2) {
            Ok(matrix) => assert_eq!(matrix, m3),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_get_determinant_3_x_3() {
        let rows = vec![vec![5, 1, -1], vec![-2, 4, 2], vec![3, 3, 3]];

        let m = Matrix::new(rows).unwrap();
        match operations::get_determinant(&m) {
            Ok(result) => {
                assert_eq!(result, 60)
            }
            _ => assert!(false),
        };
    }

    #[test]
    fn test_get_determinant_4_x_4() {
        let rows = vec![
            vec![5, 1, -1, 0],
            vec![-2, 4, 2, -1],
            vec![3, 3, 3, 3],
            vec![1, 2, 3, 4],
        ];

        let m = Matrix::new(rows).unwrap();
        match operations::get_determinant(&m) {
            Ok(result) => {
                assert_eq!(result, 66)
            }
            _ => assert!(false),
        };
    }
}
