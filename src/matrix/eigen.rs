use super::{logic::is_square, Matrix};

pub struct EigenMatrix {
    pub rows: Vec<Vec<String>>,
    pub m: usize,
    pub n: usize,
}

pub fn get_expanded_eigen_polynomial<
    T: Copy + std::fmt::Display + From<u8> + std::cmp::PartialOrd,
>(
    matrix: &Matrix<T>,
) -> Vec<String> {
    if !is_square(&matrix) {
        return vec![] as Vec<String>;
    }

    let m = matrix.m;
    let rows = &matrix.rows;
    let mut terms: Vec<String> = vec![];
    for i in 0..m {
        let diag_element = rows[i][i];
        let diag_str = diag_element.to_string();
        terms.push(format!("({}-x)", diag_str))
    }

    terms
}
