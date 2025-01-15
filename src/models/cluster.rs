use core::f64;
use rand::prelude::*;

use crate::{
    error::{CustomErrors, MismatchError},
    matrix::Matrix,
    vector::operations::{add_vec, scalar_divide},
};

#[derive(Debug)]
pub struct ClusterResult {
    _centroids: Vec<Vec<f64>>,
    _n_centroids: i32,
    _clusters: Vec<Vec<Vec<f64>>>,
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

pub fn init_clusters(n_centroids: i32) -> Vec<Vec<Vec<f64>>> {
    let mut clusters: Vec<Vec<Vec<f64>>> = vec![];

    for _ in 0..n_centroids {
        clusters.push(vec![])
    }

    clusters
}

pub fn new_centroids(clusters: &Vec<Vec<Vec<f64>>>) -> Result<Vec<Vec<f64>>, CustomErrors> {
    let mut new_centroids: Vec<Vec<f64>> = vec![];
    for c in clusters {
        let mut new_centroid: Vec<f64> = vec![];
        let mut scalar = 0.0;
        for idx in 0..c.len() {
            if idx == 0 {
                new_centroid = c[idx].to_vec();
            } else {
                new_centroid = match add_vec(&new_centroid, &c[idx]) {
                    Ok(val) => val.to_vec(),
                    Err(_) => return Err(CustomErrors::Mismatch(MismatchError)),
                }
            }
            scalar += 1.0;
        }
        new_centroid = match scalar_divide(&new_centroid, scalar) {
            Ok(val) => val.to_vec(),
            Err(err) => return Err(err),
        };

        new_centroids.push(new_centroid.to_vec());
    }
    Ok(new_centroids)
}

pub fn kmeans(data: &Matrix<f64>, n_centroids: i32) -> Result<ClusterResult, CustomErrors> {
    let mut centroids: Vec<Vec<f64>> = match kpp_init(data, n_centroids) {
        Ok(centroids) => centroids,
        Err(_) => return Err(CustomErrors::Mismatch(MismatchError)),
    };
    let mut clusters: Vec<Vec<Vec<f64>>> = vec![];
    let mut converged: bool = false;
    while !converged {
        clusters = init_clusters(n_centroids);

        for pt in &data.rows {
            let mut min_dist = f64::MAX;
            let mut min_idx: usize = 0;

            for c in 0..centroids.len() {
                let dist = match cartesian_distance(&pt, &centroids[c]) {
                    Ok(dist) => dist,
                    Err(_) => return Err(CustomErrors::Mismatch(MismatchError)),
                };

                if dist < min_dist {
                    min_dist = dist;
                    min_idx = c
                }
            }
            clusters[min_idx].push(pt.to_vec())
        }

        let new_centroids = match new_centroids(&clusters) {
            Ok(new_centroids) => new_centroids,
            Err(err) => return Err(err),
        };

        if new_centroids == centroids {
            converged = true
        } else {
            centroids = new_centroids
        }
    }

    Ok(ClusterResult {
        _centroids: centroids,
        _clusters: clusters,
        _n_centroids: n_centroids,
    })
}

#[cfg(test)]
mod tests {

    use crate::error::{CustomErrors, MismatchError};
    use crate::matrix::Matrix;
    use crate::models::cluster::kmeans;

    use super::{cartesian_distance, init_clusters, kpp_init, new_centroids};

    #[test]
    fn test_kmeans() {
        let rows: Vec<Vec<f64>> = vec![
            vec![0.0, 0.0],
            vec![0.0, 0.5],
            vec![0.5, 0.0],
            vec![4.0, 4.0],
            vec![4.5, 4.0],
            vec![4.0, 5.0],
        ];
        let target_centroids_a: Vec<Vec<f64>> = vec![
            vec![0.16666666666666666, 0.16666666666666666],
            vec![4.166666666666667, 4.333333333333333],
        ];
        let target_centroids_b = vec![
            vec![4.166666666666667, 4.333333333333333],
            vec![0.16666666666666666, 0.16666666666666666],
        ];
        let data = Matrix::new(rows).unwrap();
        let n_centroids = 2;
        let res = kmeans(&data, n_centroids).unwrap();

        assert_eq!(res._n_centroids, n_centroids);
        if res._centroids[0][0] == 0.16666666666666666 {
            assert_eq!(res._centroids, target_centroids_a)
        } else if res._centroids[0][0] == 4.166666666666667 {
            assert_eq!(res._centroids, target_centroids_b)
        } else {
            assert_eq!(res._centroids, target_centroids_b)
        }
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

    #[test]
    fn test_cluster_init() {
        let n_centroids = 3;
        let c = init_clusters(n_centroids);
        let targ_size: usize = 3;
        assert_eq!(targ_size, c.len())
    }

    #[test]
    fn test_new_centroids() {
        let clusters: Vec<Vec<Vec<f64>>> = vec![
            vec![vec![1.0, 1.0], vec![2.0, 2.0]],
            vec![vec![5.0, 5.0], vec![6.0, 6.0]],
            vec![vec![-1.0, -1.0], vec![-2.0, -2.0]],
        ];

        let target_centroids: Vec<Vec<f64>> =
            vec![vec![1.5, 1.5], vec![5.5, 5.5], vec![-1.5, -1.5]];

        let centroids = new_centroids(&clusters).unwrap();

        assert_eq!(target_centroids, centroids)
    }
}
