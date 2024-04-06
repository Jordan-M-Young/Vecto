use crate::error::{CustomErrors, MismatchError};
use crate::matrix::Matrix;
use crate::{operations, util};

pub fn solve_system<T: std::marker::Copy + Into<f64>>(
    coeff_matrix: Matrix<T>,
) -> Result<Vec<f64>, CustomErrors> {
    let m = coeff_matrix.m;
    let n = coeff_matrix.n;
    let zero_f64: f64 = 0.0;

    let rows = coeff_matrix.rows;
    if m + 1 != n {
        return Err(CustomErrors::Mismatch(MismatchError));
    }

    // unpack matrix into variable coeff matrix and
    // variable solve column

    let mut var_coeffiecient_rows: Vec<Vec<f64>> = vec![];
    let mut solve_row: Vec<f64> = vec![];

    for i in 0..m {
        let mut var_coeff_row: Vec<f64> = vec![];

        for j in 0..n {
            let new_val: f64 = rows[i][j].into();

            if j == n - 1 {
                solve_row.push(new_val)
            } else {
                var_coeff_row.push(new_val)
            }
        }
        var_coeffiecient_rows.push(var_coeff_row)
    }

    let m = var_coeffiecient_rows.len();
    let n = var_coeffiecient_rows[0].len();

    //counts valid position of 1 in a diagonalized column
    let mut counter = 0;

    // equation solve loop
    for i in 0..m {
        let mut col: Vec<f64> = vec![];
        println!("{},{:?}", i, &solve_row);

        //row normalizing
        for j in i..n {
            let element = var_coeffiecient_rows[j][i];

            if element == zero_f64 {
                col.push(var_coeffiecient_rows[j][i])
            } else if element != 1.0 {
                let new_row = match operations::scalar_divide(&var_coeffiecient_rows[j], element) {
                    Ok(new_row) => new_row,
                    Err(err) => return Err(err),
                };
                var_coeffiecient_rows[j] = new_row;
                solve_row[j] /= element;
                col.push(var_coeffiecient_rows[j][i])
            } else {
                col.push(var_coeffiecient_rows[j][i])
            }
        }
        println!("{},{:?}", i, &solve_row);

        // row subtraction
        if util::is_all_zeroes(&col) {
            continue;
        }

        if !util::vec_is_diagonalized(&col, counter) {
            // do row stuff lol
            for k in 0..m {
                if k == i {
                    continue;
                } else if var_coeffiecient_rows[k][i] == zero_f64 {
                    continue;
                } else {
                    let old_row = var_coeffiecient_rows[k].clone();
                    let mut sub_tract_row = &var_coeffiecient_rows[i];

                    if sub_tract_row[i] == zero_f64 && old_row[i] == zero_f64 {
                        continue;
                    } else if sub_tract_row[i] == zero_f64 {
                        let new_subtracter_row = match operations::add_vec(sub_tract_row, &old_row)
                        {
                            Ok(new_subtracter_row) => new_subtracter_row,
                            Err(err) => return Err(err),
                        };
                        solve_row[i] = solve_row[k] + solve_row[i];
                        var_coeffiecient_rows[i] = new_subtracter_row;
                        sub_tract_row = &var_coeffiecient_rows[i];

                        let new_row = match operations::sub_vec(&old_row, sub_tract_row) {
                            Ok(new_row) => new_row,
                            Err(err) => return Err(err),
                        };

                        solve_row[k] = solve_row[k] - solve_row[i];
                        var_coeffiecient_rows[k] = new_row;
                    } else {
                        let factor = old_row[i] / sub_tract_row[i];
                        let scaled_subtract_row =
                            match operations::scalar_multiply(sub_tract_row, factor) {
                                Ok(scaled_subtract_row) => scaled_subtract_row,
                                Err(err) => return Err(err),
                            };
                        let new_row = match operations::sub_vec(&old_row, &scaled_subtract_row) {
                            Ok(new_row) => new_row,
                            Err(err) => return Err(err),
                        };

                        solve_row[k] = solve_row[k] - (factor * solve_row[i]);
                        var_coeffiecient_rows[k] = new_row;
                    }
                }
            }
        }
        counter += 1;
    }

    println!("{:?}", var_coeffiecient_rows);
    Ok(solve_row)
}

#[cfg(test)]
mod tests {
    use crate::{
        error::{CustomErrors, MismatchError},
        matrix::Matrix,
    };

    use super::solve_system;

    #[test]
    fn test_solve_system() {
        // should succeed with a vector of 3,4,1
        let rows_1: Vec<Vec<f64>> = vec![
            vec![1.0, 4.0, 3.0, 3.0],
            vec![3.0, 1.0, 0.0, 4.0],
            vec![0.0, 0.0, 5.0, 5.0],
        ];
        let m1 = Matrix::new(rows_1).unwrap();
        match solve_system(m1) {
            Ok(solved_1) => {
                let e1: f64 = 16.0 / 11.0;
                let e2 = -4.0 / 11.0;
                let e3: f64 = 1.0;
                let comp_vec: Vec<f64> = vec![e1, e2, e3];
                assert_eq!(solved_1, comp_vec)
            }
            Err(_) => {
                assert!(false)
            }
        };

        //should fail on a mismatch error
        let rows_1: Vec<Vec<f64>> = vec![
            vec![1.0, 4.0, 3.0],
            vec![3.0, 1.0, 0.0],
            vec![0.0, 0.0, 5.0],
        ];
        let m1 = Matrix::new(rows_1).unwrap();
        match solve_system(m1) {
            Ok(_) => {
                assert!(false)
            }
            Err(err) => {
                assert_eq!(err, CustomErrors::Mismatch(MismatchError))
            }
        };
    }
}
