pub mod adjugate;
pub mod cofactor;
pub mod eigen;
pub mod inverse;
pub mod logic;
pub mod operations;
pub mod transform;
use crate::error::{CustomErrors, EmptyVectorError, NonUniformError};

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
