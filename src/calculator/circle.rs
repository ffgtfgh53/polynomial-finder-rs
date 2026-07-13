use std::iter::zip;

use peroxide::fuga::{LinearAlgebra, MutFP, Shape::Col, matrix};

use crate::calculator::float_parser;

pub fn solve_by_points(points: &Vec<[f64; 2]>) -> Result<String, String> {
    let data = solve_from_points(float_parser::split_points(&points))?;
    format_solve(data)
}

/// Reference: https://planetcalc.com/8116/
fn solve_from_points((x_vals, y_vals): (Vec<f64>, Vec<f64>)) -> Result<Vec<f64>, String>{
    if x_vals.len() != 3 && y_vals.len() != 3 {
        return Err("Wrong number of points given (required exactly 3)".to_string());
    }

    let mut a_data = Vec::with_capacity(9);

    a_data.append(&mut x_vals.clone());
    a_data.append(&mut y_vals.clone());
    a_data.append(&mut vec![0.5, 0.5, 0.5]);
    a_data.mut_map(|n| n * 2.0 );

    let a = matrix(a_data, 3, 3, Col);

    let b_data = zip(x_vals, y_vals)
        .map(|(x, y)| -1.0 * (x.powi(2) + y.powi(2)))
        .collect();

    let b = matrix(b_data, 3, 1, Col);

    let v = a.inv() * b;

    return Ok(v.into_vec());
}

fn format_solve(data: Vec<f64>) -> Result<String, String> {
    let (a, b, c) = (data[0], data[1], data[2]);
    let (x, y) = (2.0 * a, 2.0 * b);

    Ok(format!("x^2 + y^2 + {}x + {}y + {} = 0", x, y, c))
}