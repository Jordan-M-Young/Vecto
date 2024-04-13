use crate::matrix::Matrix;

pub fn can_add<T: Copy>(matrix_1: &Matrix<T>, matrix_2: &Matrix<T>) -> bool {
    if matrix_1.m != matrix_2.n {
        return false;
    }

    if matrix_1.n != matrix_2.n {
        return false;
    }

    true
}

pub fn can_multiply<T: Copy>(matrix_1: &Matrix<T>, matrix_2: &Matrix<T>) -> bool {
    if matrix_1.n != matrix_2.m {
        return false;
    }
    true
}

pub fn is_square<T: Copy>(matrix: &Matrix<T>) -> bool {
    if matrix.m != matrix.n {
        return false;
    }
    true
}
