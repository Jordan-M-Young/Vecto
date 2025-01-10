use crate::matrix::CustomErrors;
use crate::matrix::EmptyVectorError;
use crate::matrix::Matrix;
use crate::vector::operations::{mean, stddev};

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
