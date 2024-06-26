use std::ops::{Add, Mul, Sub};

use crate::error::{BadTypeError, CustomErrors, EmptyVectorError, MismatchError};

pub fn add_vec<T: Copy + Add<Output = T>>(
    vec_1: &Vec<T>,
    vec_2: &Vec<T>,
) -> Result<Vec<T>, CustomErrors> {
    let l1 = vec_1.len();
    let l2 = vec_2.len();
    let mut vec_3: Vec<T> = vec![];

    if l1 != l2 {
        return Err(CustomErrors::Mismatch(MismatchError));
    }

    for i in 0..l1 {
        vec_3.push(vec_1[i] + vec_2[i])
    }

    return Ok(vec_3);
}

pub fn sub_vec<T: Copy + Sub<Output = T>>(
    vec_1: &Vec<T>,
    vec_2: &Vec<T>,
) -> Result<Vec<T>, CustomErrors> {
    let l1 = vec_1.len();
    let l2 = vec_2.len();
    let mut vec_3: Vec<T> = vec![];

    if l1 != l2 {
        return Err(CustomErrors::Mismatch(MismatchError));
    }

    for i in 0..l1 {
        vec_3.push(vec_1[i] - vec_2[i])
    }

    return Ok(vec_3);
}

pub fn sum<T: Copy + From<u8> + Add<Output = T> + std::ops::AddAssign>(
    vec_1: &Vec<T>,
) -> Result<T, CustomErrors> {
    let l1 = vec_1.len();
    if l1 == 0 {
        return Err(CustomErrors::EmptyVector(EmptyVectorError));
    }

    let mut sum: T = match 0.try_into() {
        Ok(val) => val,
        _ => return Err(CustomErrors::BadType(BadTypeError)),
    };

    for i in 0..l1 {
        sum += vec_1[i];
    }

    return Ok(sum);
}

pub fn mean<T: Copy + Into<f64> + From<u8> + Add<Output = T> + std::ops::AddAssign>(
    vec_1: &Vec<T>,
) -> Result<f64, CustomErrors> {
    let l1 = vec_1.len();
    let n_elements: u8 = vec_1.len().try_into().unwrap();
    if l1 == 0 {
        return Err(CustomErrors::EmptyVector(EmptyVectorError));
    }

    let mut sum: T = match 0.try_into() {
        Ok(val) => val,
        _ => return Err(CustomErrors::BadType(BadTypeError)),
    };

    for i in 0..l1 {
        sum += vec_1[i];
    }

    let sum: f64 = match sum.try_into() {
        Ok(val) => val,
        _ => return Err(CustomErrors::BadType(BadTypeError)),
    };

    let n_elements: f64 = match n_elements.try_into() {
        Ok(val) => val,
        _ => return Err(CustomErrors::BadType(BadTypeError)),
    };

    let mean = sum / n_elements;
    return Ok(mean);
}

pub fn dot_product<
    T: Copy + Into<f64> + From<u8> + Mul<Output = T> + Add<Output = T> + std::ops::AddAssign,
>(
    vec_1: &Vec<T>,
    vec_2: &Vec<T>,
) -> Result<f64, CustomErrors> {
    let l1 = vec_1.len();
    let l2 = vec_2.len();

    if l1 != l2 {
        return Err(CustomErrors::Mismatch(MismatchError));
    };

    if l1 == 0 {
        return Err(CustomErrors::EmptyVector(EmptyVectorError));
    };

    let mut sum: T = match 0.try_into() {
        Ok(val) => val,
        _ => return Err(CustomErrors::BadType(BadTypeError)),
    };

    for i in 0..l1 {
        sum += vec_1[i] * vec_2[i]
    }

    let sum: f64 = match sum.try_into() {
        Ok(val) => val,
        _ => return Err(CustomErrors::BadType(BadTypeError)),
    };

    return Ok(sum);
}

pub fn magnitude<
    T: Copy
        + From<u8>
        + Add<Output = T>
        + std::ops::AddAssign
        + std::ops::Mul
        + std::ops::AddAssign<<T as std::ops::Mul>::Output>,
>(
    vec_1: &Vec<T>,
) -> Result<T, CustomErrors> {
    let l1 = vec_1.len();
    if l1 == 0 {
        return Err(CustomErrors::EmptyVector(EmptyVectorError));
    }

    let mut sum: T = match 0.try_into() {
        Ok(val) => val,
        _ => return Err(CustomErrors::BadType(BadTypeError)),
    };

    for i in 0..l1 {
        let sq = vec_1[i] * vec_1[i];
        sum += sq
    }

    return Ok(sum);
}

pub fn cosine_similarity<
    T: Copy + Into<f64> + From<u8> + Mul<Output = T> + Add<Output = T> + std::ops::AddAssign,
>(
    vec_1: &Vec<T>,
    vec_2: &Vec<T>,
) -> Result<f64, CustomErrors> {
    let magnitude_a = match magnitude(vec_1) {
        Ok(val) => {
            let val: f64 = match val.try_into() {
                Ok(val) => val,
                _ => return Err(CustomErrors::BadType(BadTypeError)),
            };
            val
        }
        Err(err) => return Err(err),
    };
    let magnitude_b = match magnitude(vec_2) {
        Ok(val) => {
            let val: f64 = match val.try_into() {
                Ok(val) => val,
                _ => return Err(CustomErrors::BadType(BadTypeError)),
            };
            val
        }
        Err(err) => return Err(err),
    };

    let dot_prod = match dot_product(vec_1, vec_2) {
        Ok(val) => val,
        Err(err) => return Err(err),
    };
    let mag_prod = magnitude_a * magnitude_b;

    Ok(dot_prod / mag_prod)
}

pub fn scalar_add<T: std::ops::Add<Output = T> + Copy>(
    vec_1: &Vec<T>,
    scalar: T,
) -> Result<Vec<T>, CustomErrors> {
    let l1 = vec_1.len();

    if l1 == 0 {
        return Err(CustomErrors::EmptyVector(EmptyVectorError));
    };

    let mut vec_2: Vec<T> = vec![];
    for i in 0..l1 {
        vec_2.push(vec_1[i] + scalar)
    }

    Ok(vec_2)
}

pub fn scalar_subtract<T: std::ops::Sub<Output = T> + Copy>(
    vec_1: &Vec<T>,
    scalar: T,
) -> Result<Vec<T>, CustomErrors> {
    let l1 = vec_1.len();

    if l1 == 0 {
        return Err(CustomErrors::EmptyVector(EmptyVectorError));
    };

    let mut vec_2: Vec<T> = vec![];
    for i in 0..l1 {
        vec_2.push(vec_1[i] - scalar)
    }

    Ok(vec_2)
}

pub fn scalar_multiply<T: std::ops::Mul<Output = T> + Copy>(
    vec_1: &Vec<T>,
    scalar: T,
) -> Result<Vec<T>, CustomErrors> {
    let l1 = vec_1.len();

    if l1 == 0 {
        return Err(CustomErrors::EmptyVector(EmptyVectorError));
    };

    let mut vec_2: Vec<T> = vec![];
    for i in 0..l1 {
        vec_2.push(vec_1[i] * scalar)
    }

    Ok(vec_2)
}

pub fn scalar_divide<T: Into<f64> + std::ops::Div<Output = T> + Copy>(
    vec_1: &Vec<T>,
    scalar: T,
) -> Result<Vec<f64>, CustomErrors> {
    let l1 = vec_1.len();

    if l1 == 0 {
        return Err(CustomErrors::EmptyVector(EmptyVectorError));
    };

    let mut vec_2: Vec<f64> = vec![];
    for i in 0..l1 {
        let new_val = vec_1[i] / scalar;
        let new_val: f64 = match new_val.try_into() {
            Ok(val) => val,
            _ => return Err(CustomErrors::BadType(BadTypeError)),
        };
        vec_2.push(new_val)
    }

    Ok(vec_2)
}
