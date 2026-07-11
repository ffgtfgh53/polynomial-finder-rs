use std::{error, io};

use peroxide::fuga::{LinearAlgebra, Scalable, Shape::Col, matrix, nearly_eq};

use crate::calculator::float_parser;

/// Get the points by (i, v), where i is the index of the message
/// and v is the value of the message
pub fn _solve_by_index(points: &Vec<String>) -> Result<String, Box<dyn error::Error>> {
    if points.is_empty() { return Ok("[No points entered]".to_string()) }
    let (x_vals, y_vals) = float_parser::_get_points_by_index(points)?;
    let solved = solve_from_points((x_vals, y_vals))?;
    Ok(format_solve(solved))
}

pub fn solve_by_points(points: &Vec<[f64;2]>) -> Result<String, Box<dyn error::Error>> {
    if points.is_empty() { return Ok("[No points entered]".to_string()) }
    let solved = solve_from_points(float_parser::split_points(points))?;
    Ok(format_solve(solved))
}

/// Generate the coeficients of the equation that satisfies the points in (x, y)
/// 
/// Reference: https://bueler.github.io/numerical/assets/slides/F24/polynonewt.pdf
fn solve_from_points((x_vals, y_vals): (Vec<f64>, Vec<f64>)) -> io::Result<Vec<f64>>{
    let n = x_vals.len();
    if n != y_vals.len() { 
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "mismatch in length for x_vals and y_vals"))
    };

    let mut a = matrix(vec![1].repeat(n), n, 1, Col);
    for i in 1..n {
        let col = x_vals.iter().map(|val| val.powi(i as i32)).collect();
        a = a.add_col(&col);
    }

    let b = matrix(y_vals, n, 1, Col);

    let v = a.inv() * b;

    Ok(v.into_vec())
}


/// Formats the coefficients into a properly formatted string
fn format_solve(exponents: Vec<f64>) -> String {
    let mut n = exponents.len();
    let mut res = String::new();

    for exp in exponents.iter().rev() {
        insert_term(&mut res, *exp, n-1);
        n -= 1
    }

    if res.is_empty() {
        res.push('0');
    }

    res
}

fn insert_term(res: &mut String, exp: f64, n: usize) {
    if nearly_eq(exp, 0.0) { 
        return; 
    } 

    if res.is_empty() {
        res.push_str(&format!(
            "{}{}", 
            format_exp(exp, false, n==0),
            get_pow_x(n)
        ))
    } else {
        res.push_str(&format!(
            "{}{}",
            format_exp(exp, true, n==0),
            get_pow_x(n)
        ));
    }
}

fn get_pow_x(n: usize) -> String {
    if n == 0 { "".to_string() }
    else if n == 1 { "x".to_string() }
    else { format!("x^{}", n) }
}

/// Check if it is an integer using `peroxide::fuga::nearly_eq`
fn is_integer(exp: f64) -> bool {
    nearly_eq(exp, exp.round())
}

fn format_exp(exp: f64, add_plus_sign: bool, is_last_element: bool) -> String {
    // checks 7 dp for integerness
    let res: String;

    if is_integer(exp) { 
        let exp = exp.round() as i64;

        if !is_last_element && exp == 1 {
            res = String::new()
        } else if !is_last_element && exp == -1 {
            res = "-".to_string()
        } else {
            res = exp.to_string();
        }
    
    } else { 
        res = format!("{:.7}", exp);
    }

    if add_plus_sign && exp > 0.0 {
        "+".to_string() + &res.to_string()
    } else {
        res
    }
}

