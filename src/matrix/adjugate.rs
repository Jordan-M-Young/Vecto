use crate::{error::CustomErrors, matrix::Matrix};

use super::cofactor::get_cofactor;

pub fn get_adjugate<
    T: Copy
        + std::ops::MulAssign
        + std::ops::MulAssign
        + std::fmt::Debug
        + std::convert::From<u8>
        + std::convert::From<i32>
        + std::ops::Mul
        + std::ops::Mul<Output = T>
        + Into<f64>
        + std::ops::AddAssign,
>(
    matrix: &Matrix<T>,
) -> Result<Matrix<T>, CustomErrors> {
    match get_cofactor(matrix) {
        Ok(cofactor_matrix) => Ok(cofactor_matrix.transpose()),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    use super::get_adjugate;

    #[test]
    fn test_get_adjugate() {
        let matrix = Matrix::new(vec![vec![1, 2, 3], vec![3, 2, 1], vec![1, 2, 3]]).unwrap();
        let target_adjugate =
            Matrix::new(vec![vec![4, 0, -4], vec![-8, 0, 8], vec![4, 0, -4]]).unwrap();
        let calculated_adjugate = get_adjugate(&matrix).unwrap();

        assert_eq!(target_adjugate, calculated_adjugate)
    }
}
