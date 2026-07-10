use std::{error, io};

use peroxide::fuga::{LinearAlgebra, Scalable, Shape::Col, matrix};

/// Get the points by (i, v), where i is the index of the message
/// and v is the value of the message
pub fn solve_by_index(points: &Vec<String>) -> Result<String, Box<dyn error::Error>> {
    let (x_vals, y_vals) = get_points_by_index(points)?;
    let solved = solve_from_points(x_vals, y_vals)?;
    Ok(format_solve(solved))
}

fn get_points_by_index(points: &Vec<String>) -> Result<(Vec<f64>, Vec<f64>), Box<dyn error::Error>> {
    let x_vals = (0..points.len()).map(|val| val as f64).collect();
    let y_vals: Vec<f64> = points.iter().map(|val| val.parse::<f64>()).collect::<Result<_, _>>()?;

    Ok((x_vals, y_vals))
}

/// Generate the equation that satisfies the points in (x, y)
/// 
/// Reference: https://bueler.github.io/numerical/assets/slides/F24/polynonewt.pdf
fn solve_from_points(x_vals: Vec<f64>, y_vals: Vec<f64>) -> io::Result<Vec<f64>>{
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

fn get_pow_x(n: usize) -> String {
    if n == 0 { "".to_string() }
    else if n == 1 { "x".to_string() }
    else { format!("x^{}", n) }
}

fn format_exp(exp: f64, add_plus_sign: bool) -> String {
    let is_integer = exp.trunc() == exp;
    let res: String;

    if is_integer { 
        if exp == 1.0 {
            res = String::new()
        } else {
            res = (exp as i64).to_string();
        }
    } else { 
        res = (exp as f32).to_string();
    }

    if add_plus_sign && exp > 0.0 {
        "+".to_string() + &res
    } else {
        res
    }
}

fn insert_term(res: &mut String, exp: f64, n: usize) {
    if exp == 0.0 { 
        return; 
    } 

    if res.is_empty() {
        res.push_str(&format!(
            "{}{}", 
            format_exp(exp, false),
            get_pow_x(n)
        ))
    } else {
        res.push_str(&format!(
            "{}{}",
            format_exp(exp, true),
            get_pow_x(n)
        ));
    }
}

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