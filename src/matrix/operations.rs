use crate::error;
use crate::matrix::logic::{can_add, can_multiply, is_square};
use crate::matrix::Matrix;
use crate::vector::operations::add_vec;
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

pub fn multiply_matrices<
    T: Copy + From<u8> + std::ops::Mul + std::ops::AddAssign<<T as std::ops::Mul>::Output>,
>(
    matrix_1: Matrix<T>,
    matrix_2: Matrix<T>,
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

//tests......
// ------------------------------
// ------------------------------

#[cfg(test)]
mod tests {
    use crate::matrix::{self, operations, Matrix};

    use super::multiply_matrices;

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
        match multiply_matrices(m, m2) {
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
