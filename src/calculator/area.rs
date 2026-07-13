use std::iter::zip;
use itertools::Itertools;

use crate::calculator::float_parser;

pub fn solve_by_points(points: &Vec<[f64;2]>) -> Result<String, String> {
    if points.len() < 3 { 
        Err("Need > 3 points to calculate area".to_string())? 
    }

    let solved = solve_from_points(float_parser::split_points(points))?;
    Ok("A = ".to_string() + &float_parser::display_float(solved))
}

/// Calculate the area via the shoelace method
fn solve_from_points((x_vals, y_vals): (Vec<f64>, Vec<f64>)) -> Result<f64, String>{
    if x_vals.len() != y_vals.len() {
        return Err("x_vals and y_vals have different lengths".to_string())
    } 

    let mut sum = 0.;

    for ((x1, y1), (x2, y2)) 
        in zip(x_vals, y_vals).circular_tuple_windows() {
        sum += x1 * y2 - x2 * y1;
    }

    Ok(sum * 0.5)
}
