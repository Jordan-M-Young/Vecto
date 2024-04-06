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
