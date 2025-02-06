use std::f64::consts;

use crate::{
    error::CustomErrors,
    matrix::{operations::multiply_matrix_vector, Matrix},
    vector::{operations::{add_vec, mean, multiply_vec, scalar_add, sub_from_scalar, sub_vec}, util::zeroes},
};

pub fn vector_logistic(vec: &Vec<f64>) -> Vec<f64> {
    let l = vec.len();
    let mut new_vec = vec![];
    for i in 0..l {
        new_vec.push(logistic(vec[i]));
    }
    new_vec

}

pub fn vector_log(vec: &Vec<f64>) -> Vec<f64> {
    let mut new_vec = vec![];
    for i in 0..vec.len() {
        new_vec.push(vec[i].ln())
    }
    new_vec
}

pub fn logistic(x: f64) -> f64 {
    return 1.0 / (1.0 + consts::E.powf(-x));
}

pub fn bce_loss(y_true: &Vec<f64>, y_pred: &Vec<f64>) -> Result<f64,CustomErrors> {
    let epsilon = 1e-9;
    let e_y_pred = match scalar_add(y_true, epsilon) {
        Ok(e_y_pred) => e_y_pred,
        Err(err) => {
            return Err(err)
        }
    };

    let log_e_y_pred = vector_log(&e_y_pred);

    let y1 = match multiply_vec(y_true, &log_e_y_pred) {
        Ok(y1) => y1,
        Err(err) => {
            return Err(err)
        }
    };

    let left2 = sub_from_scalar(1.0, y_true);
    let ypred_ep = match scalar_add(y_pred, epsilon) {
        Ok(ypred_e) => ypred_e,
        Err(err) =>  {
            return Err(err)
        }
    };
    let right2 = sub_from_scalar(1.0, &ypred_ep);
    let log_right2 = vector_log(&right2);

    let y2 = match multiply_vec(&left2, &log_right2) {
        Ok(y2) => y2,
        Err(err) => {
            return Err(err)
        }
    };

    let y = match add_vec(&y1, &y2) {
        Ok(y) => y,
        Err(err) => {
            return Err(err)
        }
    };

    let loss = match mean(&y) {
        Ok(loss) => {
            loss * - 1.0
        },
        Err(err) => {
            return Err(err)
        }
    };

    Ok(loss)




    
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

            let loss = match bce_loss(&targets, &out) {
                Ok(loss) => loss,
                Err(err) => {
                    return Err(err)
                }
            };

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
