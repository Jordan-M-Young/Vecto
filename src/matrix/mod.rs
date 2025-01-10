pub mod adjugate;
pub mod cofactor;
pub mod eigen;
pub mod inverse;
pub mod logic;
pub mod operations;
pub mod transform;
use crate::error::{CustomErrors, EmptyVectorError, NonUniformError};
use crate::vector::operations::{mean, stddev};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T: Copy> {
    pub rows: Vec<Vec<T>>,
    pub m: usize,
    pub n: usize,
}

impl<T: Copy + From<u8> + Into<f64> + std::ops::AddAssign> Matrix<T> {
    pub fn new(rows: Vec<Vec<T>>) -> Result<Matrix<T>, CustomErrors> {
        let m = rows.len();

        if m == 0 {
            return Err(CustomErrors::EmptyVector(EmptyVectorError));
        }

        let n = rows[0].len();

        if n == 0 {
            return Err(CustomErrors::EmptyVector(EmptyVectorError));
        }

        for i in 1..m {
            if rows[i].len() != n {
                return Err(CustomErrors::NonUniform(NonUniformError));
            }
        }
        Ok(Matrix { rows, m, n })
    }
    pub fn trace(&self) -> T {
        let m = self.m;
        let zero_cast: T = 0.into();
        let mut trace: T = zero_cast;
        let rows = &self.rows;
        for i in 0..m {
            trace += rows[i][i]
        }
        trace
    }

    pub fn transpose(&self) -> Matrix<T> {
        let mut new_matrix: Vec<Vec<T>> = vec![];
        let loop1: usize = self.m;
        let loop2: usize = self.n;

        for j in 0..loop2 {
            let mut transposed_row = vec![];
            for i in 0..loop1 {
                transposed_row.push(self.rows[i][j])
            }
            new_matrix.push(transposed_row)
        }
        return Matrix {
            rows: new_matrix,
            m: self.n,
            n: self.m,
        };
    }

    pub fn cast_f64(&self) -> Matrix<f64> {
        let m = self.m;
        let n = self.n;
        let rows = &self.rows;
        let mut new_rows: Vec<Vec<f64>> = vec![];
        for i in 0..m {
            let mut new_row: Vec<f64> = vec![];
            for j in 0..n {
                let new_element: f64 = rows[i][j].into();
                new_row.push(new_element)
            }
            new_rows.push(new_row)
        }

        Matrix {
            rows: new_rows,
            m,
            n,
        }
    }
}

pub fn standardize<
    T: Into<f64>
        + std::marker::Copy
        + std::convert::From<u8>
        + std::ops::Add<Output = T>
        + std::ops::Sub<f64>
        + std::fmt::Debug
        + std::ops::AddAssign,
>(
    matrix: &Matrix<T>,
) -> Result<Matrix<f64>, CustomErrors> {
    let m = matrix.m;
    let n = matrix.n;
    let rows = &matrix.rows;
    let mut new_rows = vec![];
    for j in 0..n {
        let mut new_row = vec![];
        let mut features: Vec<f64> = vec![];
        for i in 0..m {
            let v: f64 = rows[i][j].into();
            features.push(v)
        }

        let feature_mn: f64 = match mean(&features) {
            Ok(mn) => mn,
            Err(_) => return Err(CustomErrors::EmptyVector(EmptyVectorError)),
        };

        let stdd = match stddev(&features) {
            Ok(stdd) => stdd,
            Err(_) => return Err(CustomErrors::EmptyVector(EmptyVectorError)),
        };

        for k in 0..m {
            let old_val: f64 = features[k].into();
            println!("OLD {:?}  MEAN {:?} ", old_val, feature_mn);
            let mut new_val = old_val - feature_mn;
            new_val /= stdd;
            new_row.push(new_val)
        }

        new_rows.push(new_row)
    }
    let x: Matrix<f64> = Matrix::new(new_rows).unwrap();
    let xt = x.transpose();
    Ok(xt)
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    use super::standardize;

    #[test]
    fn test_standardization() {
        let rows: Vec<Vec<f64>> = vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]];

        let target_rows: Vec<Vec<f64>> = vec![
            vec![-1.224744871391589, -1.224744871391589],
            vec![0.0, 0.0],
            vec![1.224744871391589, 1.224744871391589],
        ];
        let targ_m = Matrix::new(target_rows).unwrap();
        let m = Matrix::new(rows).unwrap();

        let s = standardize(&m).unwrap();

        assert_eq!(s, targ_m)
    }
}
