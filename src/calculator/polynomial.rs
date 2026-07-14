use itertools::Itertools;
use peroxide::fuga::{
    LinearAlgebra, 
    Polynomial, 
    Scalable, 
    Shape, 
    matrix, 
    poly
};

use crate::calculator::float_parser;

/// Return the formatted polynomial that passes through all the points in `points`
/// 
/// Errors will be formatted in a String to be displayed
pub fn solve_by_points(points: &[[f64;2]]) -> Result<String, String> {
    if points.is_empty() { 
        Err("Need > 0 points to calculate".to_string())?
    }
    let solved = solve_from_points(float_parser::split_points(points))?;
    Ok("f(x) = ".to_string() + &solved.to_string())
}

/// Generate the coeficients of the equation that satisfies the points in (x, y)
/// 
/// Reference: https://bueler.github.io/numerical/assets/slides/F24/polynonewt.pdf
fn solve_from_points((x_vals, y_vals): (Vec<f64>, Vec<f64>)) -> Result<Polynomial, String>{
    let n = x_vals.len();
    if n != y_vals.len() { 
        Err("mismatch in length for x_vals and y_vals".to_string())?;
    };

    let mut a = matrix([1].repeat(n), n, 1, Shape::Col);
    for i in 1..n {
        let col = x_vals.iter().map(|val| val.powi(i as i32)).collect();
        a = a.add_col(&col);
    }

    let b = matrix(y_vals, n, 1, Shape::Col);

    let v = a.inv() * b;

    // Reduce precision to allow coercion to integers
    let v: Vec<f64> = v
        .into_vec()
        .into_iter()
        .rev()
        .skip_while(|c| c.abs() < f32::EPSILON as f64)
        .map(|c| {
            if c.is_nan() { 
                Err("No suitable polynomial found".to_string()) 
            } else { 
                Ok(c as f32 as f64) 
            }
        })
        .try_collect()?;

    Ok(poly(v))
}

