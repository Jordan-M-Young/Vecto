use std::collections::HashMap;

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

pub fn is_tridiagonal<T: Copy + From<u8> + PartialEq>(matrix: &Matrix<T>) -> bool {
    let m = matrix.m;
    let n = matrix.n;
    let rows = &matrix.rows;
    let zero_cast: T = 0.into();

    let mut valid_hash_map: HashMap<i32, usize> = HashMap::new();
    let mut count = 0;
    for j in 0..n {
        valid_hash_map.insert(count, j);
        count += 1
    }

    let mut a = -1;
    let mut b = 0;
    let mut c = 1;

    for i in 0..m {
        if valid_hash_map.contains_key(&a) {
            let a_u = valid_hash_map.get(&a).unwrap();

            if rows[i][*a_u] == zero_cast {
                return false;
            }
        }

        if valid_hash_map.contains_key(&b) {
            let b_u = valid_hash_map.get(&b).unwrap();

            if rows[i][*b_u] == zero_cast {
                return false;
            }
        }

        if valid_hash_map.contains_key(&c) {
            let c_u = valid_hash_map.get(&c).unwrap();

            if rows[i][*c_u] == zero_cast {
                return false;
            }
        }
        a += 1;
        b += 1;
        c += 1;
    }

    true
}
