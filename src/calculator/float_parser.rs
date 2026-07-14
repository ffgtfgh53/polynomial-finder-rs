use std::error;

use peroxide::fuga::choose_shorter_string;
use regex::Regex;

pub fn display_float(f: f64) -> String {
    choose_shorter_string(
        format!("{}", f as f32), 
        format!("{:.4}", f as f32)
    )
}

pub fn display_point([x, y]: [f64;2]) -> String {
    format!("({}, {})", display_float(x), display_float(y))
}

pub fn get_points(input: &str) -> Option<[f64;2]> {
    let re = Regex::new(r"(-?\d+(\.\d+)?)[^\d\.]+?(-?\d+(\.\d+)?)").unwrap(); // should always compile properly
    let cap = re.captures(input)?;

    Some([
        // Pattern match should mean parsing always success
        cap.get(1)?.as_str().parse().ok()?,
        cap.get(3)?.as_str().parse().ok()?
    ])
}

/// Splits a Vec of points to 2 Vec of x and y
pub fn split_points(points: &[[f64;2]]) -> (Vec<f64>, Vec<f64>) {
    let x_vals = points.iter().map(|point| point[0]).collect();
    let y_vals = points.iter().map(|point| point[1]).collect();

    (x_vals, y_vals)
}

pub fn _get_points_by_index(points: &[String]) -> Result<(Vec<f64>, Vec<f64>), Box<dyn error::Error>> {
    let x_vals = (0..points.len()).map(|val| val as f64).collect();
    let y_vals = points.iter().map(|val| val.parse::<f64>()).collect::<Result<_, _>>()?;

    Ok((x_vals, y_vals))
}