pub mod cluster;
use crate::matrix::{inverse::cramer_inverse, operations::multiply_matrices, Matrix};

pub fn linear_regression(x: &Matrix<f64>, y: &Matrix<f64>) -> Matrix<f64> {
    let xt = x.transpose();

    let left = multiply_matrices(&xt, &x).unwrap();
    let left = cramer_inverse(&left).unwrap();
    let right = multiply_matrices(&xt, &y).unwrap();

    let beta = multiply_matrices(&left, &right).unwrap();

    beta
}

#[cfg(test)]
mod tests {

    use crate::matrix::Matrix;

    use super::linear_regression;

    #[test]
    fn test_linear_regression() {
        let x: Vec<Vec<f64>> = vec![vec![1.0, 1.0], vec![1.0, 2.0], vec![1.0, 3.0]];
        let y: Vec<Vec<f64>> = vec![vec![1.0], vec![2.0], vec![3.0]];

        let b = vec![vec![0.0], vec![1.0]];

        let target_betas = Matrix::new(b).unwrap();
        let features = Matrix::new(x).unwrap();
        let targets = Matrix::new(y).unwrap();

        let pred_betas = linear_regression(&features, &targets);
        println!("{:?}", pred_betas);
        println!("{:?}", target_betas);

        assert_eq!(target_betas, pred_betas)
    }
}
