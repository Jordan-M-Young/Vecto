use core::f64;
use rand::prelude::*;
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

pub fn cartesian_distance(p1: &Vec<f64>, p2: &Vec<f64>) -> Result<f64, CustomErrors> {
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

pub fn kpp_init(data: &Matrix<f64>, _n_centroids: i32) -> Result<Vec<Vec<f64>>, CustomErrors> {
    let mut centroids: Vec<Vec<f64>> = vec![];
    let mut rng = thread_rng();
    let rows = &data.rows;
    let m = data.m;
    let y = rng.gen_range(0..m);
    centroids.push(rows[y].to_owned());

    for _ in 1.._n_centroids {
        let mut dists: Vec<f64> = vec![];

        for i in 0..m {
            let pt = &rows[i];
            let mut d = f64::MAX;
            let c_len = centroids.len();
            for j in 0..c_len {
                let centroid = &centroids[j];
                let dist = match cartesian_distance(pt, centroid) {
                    Ok(val) => val,
                    Err(_) => return Err(CustomErrors::Mismatch(MismatchError)),
                };
                d = f64::min(d, dist)
            }
            dists.push(d)
        }

        let mut mx: f64 = 0.0;
        let mut mx_idx: usize = 0;
        for i in 0..dists.len() {
            if dists[i] > mx {
                mx = dists[i];
                mx_idx = i;
            }
        }

        let c = rows[mx_idx].to_owned();
        centroids.push(c)
    }

    Ok(centroids)
}

pub fn kmeans(data: &Matrix<f64>, n_centroids: i32) -> Result<ClusterResult, CustomErrors> {
    let centroids: Vec<Vec<f64>> = match kpp_init(data, n_centroids) {
        Ok(centroids) => centroids,
        Err(_) => return Err(CustomErrors::Mismatch(MismatchError)),
    };
    let _clusters: HashMap<i32, Vec<Vec<f64>>> = HashMap::new();
    let dummy_result = ClusterResult {
        _centroids: centroids,
        _n_centroids: n_centroids,
        _clusters,
    };
    Ok(dummy_result)
}

#[cfg(test)]
mod tests {

    use crate::error::{CustomErrors, MismatchError};
    use crate::matrix::Matrix;
    use crate::models::cluster::kmeans;

    use super::{cartesian_distance, kpp_init};

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

        let target_d = cartesian_distance(&p1, &p2).unwrap();

        assert_eq!(target_d, d);

        let p1 = vec![1.0];
        let p2 = vec![0.0, 1.0];
        match cartesian_distance(&p1, &p2) {
            Ok(_) => 0.0,
            Err(err) => {
                assert_eq!(err, CustomErrors::Mismatch(MismatchError));
                0.0
            }
        };
    }
    #[test]
    fn test_kpp_init() {
        let rows = vec![
            vec![0.0, 0.0],
            vec![-1.0, -1.0],
            vec![2.0, 2.0],
            vec![3.0, 3.0],
        ];
        let m = Matrix::new(rows).unwrap();
        let n_centroids = 3;
        let centroids = kpp_init(&m, n_centroids).unwrap();

        let targ_len: usize = 3;
        println!("{:?}", centroids);

        assert_eq!(centroids.len(), targ_len);

        if centroids[0] == vec![0.0, 0.0] {
            assert_eq!(centroids[1], vec![3.0, 3.0])
        } else if centroids[0] == vec![3.0, 3.0] {
            assert_eq!(centroids[1], vec![-1.0, -1.0])
        } else if centroids[0] == vec![-1.0, -1.0] {
            assert_eq!(centroids[1], vec![3.0, 3.0])
        } else {
            assert_eq!(centroids[1], vec![-1.0, -1.0])
        }
    }
}
