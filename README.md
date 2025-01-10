# Vecto
A Linear Algebra library with Machine Learning aspirations. Check out what this can do below.


## Build

```shell
cargo build
```

## Add as project dependency 

Clone this repo, build the project and add the following to your project's Cargo.Toml file:

```toml
[dependencies]
vecto = { path = "<PATH_TO>/vecto/" }
```

## Use

### Vector operations
```rust
let vec_1 = vec![1.0, 1.0, 1.0];
let vec_2 = vec![1.0, 1.0, 1.0];

// add/subract vectors
let res = operations::add_vec(&vec_1, &vec_2);
let res = operations::sub_vec(&vec_1, &vec_2);

// sums, mean, magnitude
let res = operations::sum(&vec_1);
let res = operations::mean(&vec_1);
let res = operations::magnitude(&vec_1)

// scalar operations
let res = operations::scalar_add(&vec_1, 3.0);
let res = operations::scalar_multiply(&vec_1, 3.0);
let res = operations::scalar_subtract(&vec_1, 3.0);
let res = operations::scalar_divide(&vec_1, 3.0);

// dot product, cosine similarity
let res = operations::dot_product(&vec_1, &vec_2);
let res = operations::cosine_similarity(&vec_1, &vec_2);
```
### Matrix Operations

```rust
//create matrices
let vec_1 = vec![1.0, 1.0, 1.0];
let vec_2 = vec![1.0, 1.0, 1.0];

let vec3 = vec![1.0, 1.0];
let vec4 = vec![1.0, 1.0];
let vec5 = vec![1.0, 1.0];

let matrix_a = matrix::Matrix::new(vec![vec_1,vec_2])
let matrix_b = matrix::Matrix::new(vec![vec_3,vec_4,vec_5])
let matrix_c = let matrix_a = matrix::Matrix::new(vec![vec_1,vec_2])

//transpose matrix
let transposed_matrix = matrix_a.transpose()

//identity matrices
let size: usize = 2
let identity_2_by_2 = matrix::create_identity_matrix(size)

//add matrices
let matrix_d = matrix::add_matrices(matrix_a, matrix_b)

// multiply matrices
let matrix_d = matrix::multiply_matrices(matrix_a,matrix_b)

// get matrix determinant
let rows = vec![vec![1, 1, -1], vec![-2, 4, 2], vec![3, 3, 3]];
let m = Matrix::new(rows).unwrap();
let determinant = matrix::get_determinant(&m);

// get matrix inverse
let rows = vec![vec![1, 1, 1], vec![3, 2, 1], vec![2, 1, 3]];
let m = Matrix::new(rows).unwrap();
let inverse = cramer_inverse(&m);

```

### Linear Algebra

```rust

// solve system of equations
let rows_1: Vec<Vec<f64>> = vec![
    vec![1.0, 4.0, 3.0, 3.0],
    vec![3.0, 1.0, 0.0, 4.0],
    vec![0.0, 0.0, 5.0, 5.0],
];
let matrix = Matrix::new(rows_1).unwrap();
let solved = algebra::solve_system(matrix);

//tridiagonalize matrix
let rows_1: Vec<Vec<f64>> = vec![
    vec![4.0, 1.0, -2.0, 2.0],
    vec![1.0, 2.0, 0.0, 1.0],
    vec![-2.0, 0.0, 3.0, -2.0],
    vec![2.0, 1.0, -2.0, -1.0],
];
let matrix = Matrix::new(rows_1).unwrap();
let m1 = tridiagonalize(&matrix);

```
### Machine Learning

```rust 

// linear regression
let x: Vec<Vec<f64>> = vec![
    vec![1.0, 1.0],
    vec![1.0, 2.0],
    vec![1.0, 3.0]
];
let y: Vec<Vec<f64>> = vec![vec![1.0],vec![2.0], vec![3.0]];

let features = Matrix::new(x).unwrap();
let targets = Matrix::new(y).unwrap();

let betas = models::linear_regression(&features, &targets);

println!("{:?}",betas)



```


### Polynomials

```rust

// get roots
let equation = "3x - 2".to_string();
let poly = Polynomial::new(equation);
let roots = poly.get_roots();

```









