use std::collections::HashMap;

use crate::{error::CustomErrors, matrix::Matrix};

pub struct ClusterResult {
    _centroids: Vec<Vec<f64>>,
    _n_centroids: i32,
    _clusters: HashMap<i32, Vec<Vec<f64>>>,

}


pub fn kmeans(_data: &Matrix<f64>, _n_centroids: i32) -> Result<ClusterResult, CustomErrors> {
    let _centroids: Vec<Vec<f64>> = vec![];
    let _clusters: HashMap<i32, Vec<Vec<f64>>> = HashMap::new();
    let dummy_result= ClusterResult {
        _centroids,    
        _n_centroids,
        _clusters
    };
    Ok(dummy_result)
}

#[cfg(test)]
mod tests {

    use crate::matrix::Matrix;
    use crate::models::cluster::kmeans;

    #[test]
    fn test_kmeans() {
        let rows: Vec<Vec<f64>> = vec![vec![1.0,1.0]];
        let data = Matrix::new(rows).unwrap();
        let n_centroids = 3;
        let res = kmeans(&data, n_centroids).unwrap();


        assert_eq!(res._n_centroids,n_centroids)
    }
}


