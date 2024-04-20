use crate::{
    error::{CustomErrors, NonUniformError},
    matrix::{logic::is_tridiagonal, Matrix},
};

use super::householder::house_holder_transform;

pub fn tridiagonalize<
    T: Copy
        + Into<f64>
        + std::cmp::PartialOrd
        + std::convert::From<f64>
        + From<u8>
        + std::cmp::PartialEq
        + std::ops::AddAssign,
>(
    matrix: &Matrix<T>,
) -> Result<Matrix<f64>, CustomErrors> {
    // if is_tridiagonal(matrix) {
    //     return *matrix
    // }

    let m = matrix.m;
    let mut k = 1;
    let new_matrix = house_holder_transform(&matrix, k).unwrap();
    println!("k ={}, {:?}", k, new_matrix);

    k += 1;

    for _ in 1..m {
        let new_matrix = house_holder_transform(&new_matrix, k).unwrap();
        if is_tridiagonal(&new_matrix) {
            return Ok(new_matrix);
        }
        println!("k ={}, {:?}", k, new_matrix);

        k += 1;
    }

    Err(CustomErrors::NonUniform(NonUniformError))
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    use super::tridiagonalize;

    #[test]
    fn test_tridiagnolize() {
        let rows_1: Vec<Vec<f64>> = vec![
            vec![4.0, 1.0, -2.0, 2.0],
            vec![1.0, 2.0, 0.0, 1.0],
            vec![-2.0, 0.0, 3.0, -2.0],
            vec![2.0, 1.0, -2.0, -1.0],
        ];
        let matrix = Matrix::new(rows_1).unwrap();

        let calculated_tridiagonalized = tridiagonalize(&matrix).unwrap();
        let target_tridiagonalized = Matrix {
            rows: vec![
                vec![
                    4.0,
                    -3.000000000000001,
                    1.332267629550197e-16,
                    -9.325873406851313e-16,
                ],
                vec![
                    -3.000000000000001,
                    3.3333333333333357,
                    -1.666666666666667,
                    -2.220446049250313e-16,
                ],
                vec![
                    1.332267629550197e-16,
                    -1.6666666666666665,
                    -1.3200000000000016,
                    0.9066666666666631,
                ],
                vec![
                    -9.325873406851313e-16,
                    0.0,
                    0.9066666666666627,
                    1.986666666666669,
                ],
            ],
            m: 4,
            n: 4,
        };

        assert_eq!(calculated_tridiagonalized, target_tridiagonalized)
    }
}
