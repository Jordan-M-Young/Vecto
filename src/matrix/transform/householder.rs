use crate::{error::CustomErrors, matrix::{operations::{multiply_matrices, scalar_multiply, sub_matrices}, Matrix}, vector::util::create_identity_matrix};

pub fn get_sign<T: From<u8> + std::cmp::PartialOrd>(element: T) -> f64 {
    let mut sign: f64 = 1.0;
    let zero_cast: T = 0.into();
    if element < zero_cast {
        sign = -1.0;
    }

    -sign
}

pub fn gen_alpha<T: Copy + From<u8> + Into<f64> + std::cmp::PartialOrd>(
    matrix: &Matrix<T>,
    k: usize,
) -> f64 {
    let rows = &matrix.rows;
    let m = matrix.m;
    let element = rows[k][k - 1];
    let sign = get_sign(element);

    let mut sum = 0.0;
    for j in k..m {
        let next: f64 = rows[j][k - 1].into();
        sum += next.powf(2.0);
    }

    sign * sum.powf(0.5)

}

pub fn get_r<T: Copy + Into<f64>>(matrix: &Matrix<T>, alpha: f64, k: usize) -> f64 {
    let rows = &matrix.rows;
    let element: f64 = rows[k][k-1].into();
    let inner_term = 0.5 * (alpha.powf(2.0) - (element * alpha));
    inner_term.powf(0.5)
}   


pub fn construct_hh_vector<T: Copy + Into<f64>>(matrix: &Matrix<T>, alpha: f64, r: f64, k: usize) -> Vec<f64> {
    let rows = &matrix.rows;
    let m = matrix.m;
    let v1 = 0.0;
    let e2: f64 = rows[k][k-1].into();
    let v2 = (e2 - alpha) / (2.0 * r);

    let mut hh_vector: Vec<f64> = vec![v1,v2];
    for j in 2..m {
        let ej: f64 = rows[j][k-1].into();
        hh_vector.push(ej / (2.0 * r))
    }

    hh_vector


}


pub fn house_holder_transform<T: Copy + Into<f64> + std::cmp::PartialOrd + From<u8> + From<f64>>(matrix: &Matrix<T>) -> Result<Matrix<f64>, CustomErrors> {
    let k = 1;
    let alpha = gen_alpha(&matrix, k);
    let r = get_r(&matrix, alpha, k);
    let hh_row = construct_hh_vector(matrix, alpha, r, k);

    let mut hh_vector: Vec<Vec<f64>> = vec![];
    let m = hh_row.len();
    for i in 0..m {
        hh_vector.push(vec![hh_row[i]])
    }
    let v = match Matrix::new(hh_vector) {
        Ok(v) => v,
        Err(err) => {return Err(err)}
    };
    let vt =  match Matrix::new(vec![hh_row]){
        Ok(vt) => vt,
        Err(err) => {return Err(err)}
    };

    let v_prod =  match multiply_matrices(v, vt) {
        Ok(v_prod) => v_prod,
        Err(err) => {return Err(err)}
    };

    let scaled_v_prod =  match scalar_multiply(&v_prod, 2.0) {
        Ok(scaled_v_prd) => scaled_v_prd,
        Err(err) => {return Err(err)}
    };


    let identity_matrix = match create_identity_matrix(matrix.m) {
        Ok(identity_matrix) => identity_matrix,
        Err(err) => {return Err(err)}
    };

    println!("{:?}",scaled_v_prod);

    sub_matrices(identity_matrix, scaled_v_prod)






}
