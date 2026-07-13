use peroxide::fuga::{LinearAlgebra, Scalable, Shape::Col, matrix};

use crate::calculator::float_parser;

/// Get the points by (i, v), where i is the index of the message
/// and v is the value of the message
pub fn _solve_by_index(points: &Vec<String>) -> Result<String, String> {
    if points.is_empty() { return Ok("[No points entered]".to_string()) }
    let (x_vals, y_vals) = float_parser::_get_points_by_index(points).map_err(|_| "Number parse error".to_string())?;
    let solved = solve_from_points((x_vals, y_vals))?;
    format_solve(solved)
}

pub fn solve_by_points(points: &Vec<[f64;2]>) -> Result<String, String> {
    if points.is_empty() { return Ok("[No points entered]".to_string()) }
    let solved = solve_from_points(float_parser::split_points(points))?;
    format_solve(solved)
}

/// Generate the coeficients of the equation that satisfies the points in (x, y)
/// 
/// Reference: https://bueler.github.io/numerical/assets/slides/F24/polynonewt.pdf
fn solve_from_points((x_vals, y_vals): (Vec<f64>, Vec<f64>)) -> Result<Vec<f64>, String>{
    let n = x_vals.len();
    if n != y_vals.len() { 
        return Err("mismatch in length for x_vals and y_vals".to_string());
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
fn format_solve(exponents: Vec<f64>) -> Result<String, String> {
    let mut n = exponents.len();
    let mut res = String::new();

    for exp in exponents.iter().rev() {
        float_parser::coefficients::insert_term(&mut res, *exp, n-1)?;
        n -= 1
    }

    if res.is_empty() {
        res.push('0');
    }

    res.insert_str(0, "f(x)=");

    Ok(res)
}

