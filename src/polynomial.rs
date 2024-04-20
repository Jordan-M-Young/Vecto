use std::collections::HashMap;

#[derive(Debug)]
pub struct Polynomial {
    order: u8,
    coeffs: HashMap<String, i32>,
    _equation: String,
    _terms: Vec<String>,
}

impl Polynomial {
    pub fn new(equation: String) -> Polynomial {
        parse_equation(&equation)
    }

    pub fn get_roots(&self) -> Vec<f64> {
        match self.order {
            0 => vec![] as Vec<f64>,
            1 => order_one_root(&self),
            2 => order_two_root(&self),
            _ => vec![] as Vec<f64>,
        }
    }
}

pub fn order_one_root(polynomial: &Polynomial) -> Vec<f64> {
    let mut roots: Vec<f64> = vec![];
    let coeffs = &polynomial.coeffs;

    let a = coeffs.get("1").unwrap();
    let b = coeffs.get("0").unwrap();

    let a = *a;
    let b = *b;

    let a: f64 = a.into();
    let b: f64 = b.into();

    let root = -b / a;

    roots.push(root);

    roots
}

pub fn order_two_root(polynomial: &Polynomial) -> Vec<f64> {
    let mut roots: Vec<f64> = vec![];
    let coeffs = &polynomial.coeffs;

    let a = coeffs.get("2").unwrap();
    let b = coeffs.get("1").unwrap();
    let c = coeffs.get("0").unwrap();

    // sqrt term ----> sqrt(b^2 - 4ac)
    let sqrt_term = b.pow(2) - 4 * a * c;
    let mut sqrt_term: f64 = sqrt_term.into();
    sqrt_term = sqrt_term.sqrt();

    // -b
    let neg_b: f64 = (-1 * b).into();

    // 2a
    let denominator: f64 = (2 * a).into();

    //root 1
    let root1 = (neg_b + sqrt_term) / denominator;

    //root 2
    let root2 = (neg_b - sqrt_term) / denominator;

    roots.push(root1);
    roots.push(root2);

    roots
}

pub fn parse_equation(equation: &str) -> Polynomial {
    let terms = get_terms(equation);
    let order = get_order(&terms);
    let coeffs = get_coeffs(&terms);
    let simplified_eq = get_simplified_equation(&coeffs);

    Polynomial {
        order,
        coeffs,
        _equation: simplified_eq,
        _terms: terms,
    }
}

pub fn get_simplified_equation(coeffs: &HashMap<String, i32>) -> String {
    //todo. build a string representation of the simplified input polynomial
    // i.e. x^2 + 3x + 2 - x -> x^2 + 2x + 2

    let mut simplified_equation = "".to_string();

    // might be worth using the itertools library to sort terms in order of polynomial order aka key
    let keys: Vec<&String> = coeffs.keys().collect();
    for key in keys {
        match coeffs.get(key) {
            Some(coeff) => {
                let mut operator = "+".to_string();
                let mut coeff_str = "".to_string();

                //only here to quiet the warning of line let mut coeff_str = "".to_string();
                let _ = coeff_str;

                if *coeff < 0 {
                    operator = "-".to_string()
                }

                if key == "0" {
                    coeff_str = coeff.to_string();
                } else if key == "1" {
                    coeff_str = format!("{}x", coeff.to_string());
                } else {
                    coeff_str = format!("{}x^{}", coeff.to_string(), key);
                }

                if simplified_equation.is_empty() {
                    simplified_equation = coeff_str;
                } else {
                    coeff_str = coeff_str.replace("-", "");
                    let term = format!("{} {}", operator, coeff_str);
                    simplified_equation = format!("{} {}", simplified_equation, term)
                }
            }
            None => {}
        };
    }

    simplified_equation
}

pub fn get_coeffs(terms: &Vec<String>) -> HashMap<String, i32> {
    let mut coeffs: HashMap<String, i32> = HashMap::new();
    for term in terms {
        let split_term: Vec<&str> = term.split("x").collect();

        let coeff = match split_term[0] {
            "" => 1,
            "-" => -1,
            _ => split_term[0].parse().unwrap(),
        };

        if split_term.len() == 1 {
            // order = 0
            coeffs
                .entry("0".to_string())
                .and_modify(|e| *e += coeff)
                .or_insert(coeff);
            continue;
        }

        match split_term[1] {
            "" => {
                coeffs
                    .entry("1".to_string())
                    .and_modify(|e| *e += coeff)
                    .or_insert(coeff);
            }
            _ => {
                let order = split_term[1].replace("^", "");
                coeffs
                    .entry(order.to_string())
                    .and_modify(|e| *e += coeff)
                    .or_insert(coeff);
            }
        }
    }

    coeffs
}

pub fn get_order(terms: &Vec<String>) -> u8 {
    let mut order: u8 = 0;
    for term in terms {
        if term.contains("x") && !term.contains("^") {
            let poly = 1;
            if order < poly {
                order = poly
            }
            continue;
        }

        let chars = term.chars();
        let mut last_char: char = '.';
        let mut last_last_char: char = '.';
        for char in chars {
            if last_char == '^' && char == '-' {
                last_last_char = '^';
                last_char = '-';
                continue;
            }

            if last_char == '-' && last_last_char == '^' {
                let poly = char.to_string();
                let poly: u8 = poly.parse().unwrap();

                if poly > order {
                    order = poly
                }
                continue;
            }

            if last_char == '^' {
                let poly = char.to_string();
                let poly: u8 = poly.parse().unwrap();

                if poly > order {
                    order = poly
                }
                continue;
            }
            last_last_char = last_char;
            last_char = char;
        }
    }
    order
}

pub fn get_terms(equation: &str) -> Vec<String> {
    let mut equation = equation.replace(" ", "");
    equation = equation.replace("+", " ").replace("-", " -");
    let split_equation: Vec<&str> = equation.split(" ").collect();
    let mut terms: Vec<String> = vec![];
    for term in split_equation {
        if term != "" {
            terms.push(term.to_owned())
        }
    }
    terms
}
