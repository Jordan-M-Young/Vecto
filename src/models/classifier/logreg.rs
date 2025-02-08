use std::f64::consts;

use crate::{
    error::CustomErrors,
    matrix::{operations::multiply_matrix_vector, Matrix},
    vector::{
        operations::{
            add_vec, mean, multiply_vec, scalar_add, scalar_multiply, sub_from_scalar, sub_vec, sum,
        },
        util::zeroes,
    },
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

pub fn bce_loss(y_true: &Vec<f64>, y_pred: &Vec<f64>) -> Result<f64, CustomErrors> {
    let epsilon = 1e-9;
    let e_y_pred = match scalar_add(y_true, epsilon) {
        Ok(e_y_pred) => e_y_pred,
        Err(err) => return Err(err),
    };

    let log_e_y_pred = vector_log(&e_y_pred);

    let y1 = match multiply_vec(y_true, &log_e_y_pred) {
        Ok(y1) => y1,
        Err(err) => return Err(err),
    };

    let left2 = sub_from_scalar(1.0, y_true);
    let ypred_ep = match scalar_add(y_pred, epsilon) {
        Ok(ypred_e) => ypred_e,
        Err(err) => return Err(err),
    };
    let right2 = sub_from_scalar(1.0, &ypred_ep);
    let log_right2 = vector_log(&right2);

    let y2 = match multiply_vec(&left2, &log_right2) {
        Ok(y2) => y2,
        Err(err) => return Err(err),
    };

    let y = match add_vec(&y1, &y2) {
        Ok(y) => y,
        Err(err) => return Err(err),
    };

    let loss = match mean(&y) {
        Ok(loss) => loss * -1.0,
        Err(err) => return Err(err),
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
                Err(err) => return Err(err),
            };

            self.losses.push(loss);

            let dz = match sub_vec(&out, &targets) {
                Ok(dz) => dz,
                Err(err) => return Err(err),
            };

            let n_samples_f64: f64 = n_samples as f64;

            let dw_l = 1.0 / n_samples_f64;
            let features_t = features.transpose();
            let dw_r = match multiply_matrix_vector(&features_t, &dz) {
                Ok(dw_r) => dw_r,
                Err(err) => return Err(err),
            };
            let dw = match scalar_multiply(&dw_r, dw_l) {
                Ok(dw) => dw,
                Err(err) => return Err(err),
            };

            let sum_dz = match sum(&dz) {
                Ok(sum_dz) => sum_dz,
                Err(err) => return Err(err),
            };
            let db = (1.0 / n_samples_f64) * sum_dz;

            let weight_increment = match scalar_multiply(&dw, self.lr) {
                Ok(weight_increment) => weight_increment,
                Err(err) => return Err(err),
            };
            self.weights = match sub_vec(&self.weights, &weight_increment) {
                Ok(new_weights) => new_weights,
                Err(err) => return Err(err),
            };

            self.bias = self.bias - (self.lr * db)
        }
        let output = self.losses.clone();
        Ok(output)
    }
    pub fn feed_forward(&self, features: &Matrix<f64>) -> Result<Vec<f64>, CustomErrors> {
        let z = match multiply_matrix_vector(features, &self.weights) {
            Ok(z) => z,
            Err(err) => return Err(err),
        };

        let a = vector_logistic(&z);

        Ok(a)
    }
    pub fn predict_prob(&self, features: &Matrix<f64>) -> Result<Vec<f64>, CustomErrors> {
        let y_hat = match multiply_matrix_vector(&features, &self.weights) {
            Ok(y_hat) => y_hat,
            Err(err) => return Err(err),
        };
        let y_pred = vector_logistic(&y_hat);
        Ok(y_pred)
    }
    pub fn predict(&self, features: &Matrix<f64>) -> Result<Vec<f64>, CustomErrors> {
        let threshold = 0.5;
        let y_pred = match self.predict_prob(features) {
            Ok(y_pred) => y_pred,
            Err(err) => return Err(err),
        };

        let mut pred_class: Vec<f64> = vec![];
        for i in 0..y_pred.len() {
            if y_pred[i] > threshold {
                pred_class.push(1.0)
            } else {
                pred_class.push(0.0)
            }
        }
        Ok(pred_class)
    }
}


#[cfg(test)]
mod tests {
    use super::{logistic, vector_logistic};



    #[test]
    fn test_logistic() {
        let input = 1.0;

        let assumed = 0.7310585786300049;

        let output = logistic(input);
    
        assert_eq!(assumed, output)



    }

    #[test]
    fn test_vector_logistic() {
        let input: Vec<f64> = vec![
            1.0, 2.0, 3.0
        ];

        let assumed: Vec<f64> = vec![
            0.7310585786300049,
            0.8807970779778823,
            0.9525741268224331,
        ];

        let output = vector_logistic(&input);

        assert_eq!(assumed,output)
    }
}