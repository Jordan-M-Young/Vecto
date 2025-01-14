use std::collections::HashMap;

use crate::{
    error::{CustomErrors, MismatchError},
    matrix::Matrix,
};

pub struct ClusterResult {
    _centroids: Vec<Vec<f64>>,
    _n_centroids: i32,
    _clusters: HashMap<i32, Vec<Vec<f64>>>,
}

pub fn cartesian_distance(p1: Vec<f64>, p2: Vec<f64>) -> Result<f64, CustomErrors> {
    let mut sqsum = 0.0;

    if p1.len() != p2.len() {
        return Err(CustomErrors::Mismatch(MismatchError));
    };

    for i in 0..p1.len() {
        sqsum += (p1[i] - p2[i]).powf(2.0)
    }

    Ok(sqsum.powf(0.5))

    // return np.sqrt(np.sum((p1 - p2)**2))
}

pub fn kmeans(_data: &Matrix<f64>, _n_centroids: i32) -> Result<ClusterResult, CustomErrors> {
    let _centroids: Vec<Vec<f64>> = vec![];
    let _clusters: HashMap<i32, Vec<Vec<f64>>> = HashMap::new();
    let dummy_result = ClusterResult {
        _centroids,
        _n_centroids,
        _clusters,
    };
    Ok(dummy_result)
}

#[cfg(test)]
mod tests {

    use crate::error::{CustomErrors, MismatchError};
    use crate::matrix::Matrix;
    use crate::models::cluster::kmeans;

    use super::cartesian_distance;

    #[test]
    fn test_kmeans() {
        let rows: Vec<Vec<f64>> = vec![vec![1.0, 1.0]];
        let data = Matrix::new(rows).unwrap();
        let n_centroids = 3;
        let res = kmeans(&data, n_centroids).unwrap();

        assert_eq!(res._n_centroids, n_centroids)
    }

    #[test]
    fn test_cartesian_distance() {
        let p1 = vec![1.0, 0.0];
        let p2 = vec![0.0, 1.0];

        let mut d: f64 = 2.0;
        d = d.powf(0.5);

        let target_d = cartesian_distance(p1, p2).unwrap();

        assert_eq!(target_d, d);

        let p1 = vec![1.0];
        let p2 = vec![0.0, 1.0];
        match cartesian_distance(p1, p2) {
            Ok(_) => 0.0,
            Err(err) => {
                assert_eq!(err, CustomErrors::Mismatch(MismatchError));
                0.0
            }
        };
    }
}
