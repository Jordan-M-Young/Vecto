use crate::error::CustomErrors;
use crate::error::EmptyVectorError;
use crate::matrix::Matrix;
use std::fmt::Debug;

pub fn vec_is_diagonalized<T: From<u8> + std::cmp::PartialEq>(
    vec: &Vec<T>,
    position: usize,
) -> bool {
    let len = vec.len();
    let zero_cast: T = 0.into();
    let one_cast: T = 1.into();

    for i in 0..len {
        if i == position && vec[i] == one_cast {
            continue;
        } else if i == position && vec[i] != one_cast {
            return false;
        } else if i != position && vec[i] == zero_cast {
            continue;
        } else {
            return false;
        }
    }
    return true;
}

pub fn is_all_zeroes<T: From<u8> + std::cmp::PartialEq>(vec: &Vec<T>) -> bool {
    let cast_zero: T = 0.into();
    let len = vec.len();
    for i in 0..len {
        if vec[i] != cast_zero {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use crate::util::{is_all_zeroes, vec_is_diagonalized};
    #[test]
    fn test_vec_is_diagonalized() {
        let vec1 = vec![0, 0, 1];
        let vec2 = vec![0, 1, 0];
        let vec3 = vec![1, 0, 0];

        assert_eq!(vec_is_diagonalized(&vec1, 2), true);
        assert_eq!(vec_is_diagonalized(&vec1, 1), false);
        assert_eq!(vec_is_diagonalized(&vec2, 0), false);
        assert_eq!(vec_is_diagonalized(&vec2, 1), true);
        assert_eq!(vec_is_diagonalized(&vec3, 1), false);
        assert_eq!(vec_is_diagonalized(&vec3, 0), true);
    }

    #[test]
    fn test_is_all_zeros() {
        let vec1 = vec![0, 0, 1];
        let vec2 = vec![0, 0, 0];
        let vec3: Vec<f32> = vec![0.0, 0.0, 0.0];
        let vec4: Vec<f64> = vec![0.0, 0.0, 0.0];
        let vec5: Vec<f64> = vec![2.0, 0.0, 0.0];

        assert_eq!(is_all_zeroes(&vec1), false);
        assert_eq!(is_all_zeroes(&vec2), true);
        assert_eq!(is_all_zeroes(&vec3), true);
        assert_eq!(is_all_zeroes(&vec4), true);
        assert_eq!(is_all_zeroes(&vec5), false);
    }
}

pub fn get_perms(size: usize) -> Vec<Vec<usize>> {
    let mut vec: Vec<usize> = vec![];
    for i in 0..size {
        vec.push(i);
    }

    heap_permutation(&mut vec, size)
}

pub fn heap_permutation<T: Copy + std::fmt::Debug>(vec: &mut Vec<T>, size: usize) -> Vec<Vec<T>> {
    let mut permutations: Vec<Vec<T>> = vec![];

    if size == 1 {
        return vec![vec.to_vec()];
    }

    for i in 0..size {
        let heap = heap_permutation(vec, size - 1);

        for p in heap {
            permutations.push(p)
        }

        if size.rem_euclid(2) == 1 {
            let a = vec[0];
            let b = vec[size - 1];
            vec[0] = b;
            vec[size - 1] = a;
        } else {
            let a = vec[i];
            let b = vec[size - 1];
            vec[i] = b;
            vec[size - 1] = a;
        }
    }
    permutations
}

pub fn get_permutation_sign<T: Debug + Copy + std::cmp::PartialOrd>(mut perm: Vec<T>) -> i32 {
    let operation = modified_bubble_sort(&mut perm);
    if operation.rem_euclid(2) == 0 {
        return 1;
    } else {
        return -1;
    }
}

pub fn modified_bubble_sort<T: Copy + std::cmp::PartialOrd>(vec: &mut Vec<T>) -> i32 {
    let mut operations = 0;
    let n = vec.len();

    for i in 0..n {
        let mut swapped = false;
        let upper_bound = n - i - 1;
        for j in 0..upper_bound {
            if vec[j] > vec[j + 1] {
                let a = vec[j];
                let b = vec[j + 1];
                vec[j + 1] = a;
                vec[j] = b;
                operations += 1;
                swapped = true
            }
        }

        if !swapped {
            return operations;
        }
    }

    operations
}

pub fn create_identity_matrix<T: Copy + From<u8>>(dim: usize) -> Result<Matrix<T>, CustomErrors> {
    if dim == 0 {
        return Err(CustomErrors::EmptyVector(EmptyVectorError));
    }
    let cast_zero: T = 0.into();
    let cast_one: T = 1.into();

    let mut rows: Vec<Vec<T>> = vec![];
    for i in 0..dim {
        let mut matrix_row: Vec<T> = vec![];

        for j in 0..dim {
            if j == i {
                matrix_row.push(cast_one)
            } else {
                matrix_row.push(cast_zero)
            }
        }
        rows.push(matrix_row);
    }
    Ok(Matrix {
        rows,
        m: dim,
        n: dim,
    })
}
