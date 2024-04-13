use crate::error::{CustomErrors, SingularMatrixError};

use super::{
    adjugate::get_adjugate,
    operations::{get_determinant, scalar_divide},
    Matrix,
};

pub fn cramer_inverse<
    T: Copy
        + From<u8>
        + std::ops::MulAssign
        + std::fmt::Debug
        + std::convert::From<u8>
        + std::convert::From<i32>
        + std::ops::Mul
        + std::ops::Mul<Output = T>
        + std::ops::Div
        + std::ops::Div<Output = T>
        + std::ops::AddAssign
        + PartialEq
        + Into<f64>,
>(
    matrix: &Matrix<T>,
) -> Result<Matrix<f64>, CustomErrors> {
    let zero_cast: T = 0.into();
    let det = match get_determinant(&matrix) {
        Ok(det) => det,
        Err(err) => return Err(err),
    };

    if det == zero_cast {
        return Err(CustomErrors::SingularMatrix(SingularMatrixError));
    }

    let adjugate_matrix = match get_adjugate(matrix) {
        Ok(adjugate_matrix) => adjugate_matrix,
        Err(err) => return Err(err),
    };
    println!("{:?}", &adjugate_matrix);
    match scalar_divide(&adjugate_matrix, det) {
        Ok(inverse_matrix) => Ok(inverse_matrix),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    use super::cramer_inverse;

    #[test]
    fn test_cramer_inverse() {
        let rows = vec![vec![1, 1, 1], vec![3, 2, 1], vec![2, 1, 3]];

        let m = Matrix::new(rows).unwrap();
        let target_inverse = Matrix::new(vec![
            vec![-1.6666666666666667, 0.6666666666666666, 0.3333333333333333],
            vec![2.3333333333333335, -0.3333333333333333, -0.6666666666666666],
            vec![0.3333333333333333, -0.3333333333333333, 0.3333333333333333],
        ])
        .unwrap();
        let calculated_inverse = cramer_inverse(&m).unwrap();

        assert_eq!(target_inverse, calculated_inverse)
    }
}
