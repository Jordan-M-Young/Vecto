use std::f64::consts;

use crate::{
    error::CustomErrors,
    matrix::{operations::multiply_matrix_vector, Matrix},
    vector::util::zeroes,
};

pub fn vector_logistic(vec: &Vec<f64>) -> Vec<f64> {
    let l = vec.len();
    let mut new_vec = vec![];
    for i in 0..l {
        new_vec.push(logistic(vec[i]));
    }
    new_vec
}

pub fn logistic(x: f64) -> f64 {
    return 1.0 / (1.0 + consts::E.powf(-x));
}

pub fn bce_loss(y_true: Vec<f64>, y_pred: Vec<f64>) -> f64 {
    let epsilon = 1e-9;
    0.0
}

pub struct LogisticRegression {
    lr: f64,
    n_iterations: u32,
    losses: Vec<f64>,
    bias: f64,
    weights: Vec<f64>,
}

impl LogisticRegression {
    pub fn new() -> LogisticRegression {
        LogisticRegression {
            lr: 0.0001,
            n_iterations: 1000,
            losses: vec![],
            bias: 0.0,
            weights: vec![],
        }
    }

    pub fn fit(
        &mut self,
        features: Matrix<f64>,
        targets: Vec<f64>,
    ) -> Result<Vec<f64>, CustomErrors> {
        let n_samples = features.m;
        let n_features = features.n;

        self.weights = zeroes(n_features);

        for _ in 0..self.n_iterations {
            let out = match self.feed_forward(&features) {
                Ok(out) => out,
                Err(err) => return Err(err),
            };

            // loss = compute_loss(y, out)

            // losses.push(loss)

            // dz = out - y

            // dw = (1 / n_samples) * (features.transpos() dot dz)

            // db = (1 / n_samples) * sum(dz)

            // self.weights = self.weights - (self.lr * dw)

            // self.bias =  self.bias - (self.lr * db)
        }
        Ok(vec![])
    }
    pub fn feed_forward(&self, features: &Matrix<f64>) -> Result<Vec<f64>, CustomErrors> {
        let z = match multiply_matrix_vector(features, &self.weights) {
            Ok(z) => z,
            Err(err) => return Err(err),
        };

        let a = vector_logistic(&z);

        Ok(a)
    }
}
