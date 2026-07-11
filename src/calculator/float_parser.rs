use std::error;
use regex::Regex;

pub fn get_points(input: &String) -> Option<[f64;2]> {
    let re = Regex::new(r"(\d+(\.\d+)?)[^\d\.]+(\d+(\.\d+)?)").unwrap(); // should always compile properly
    let cap = re.captures(input)?;

    Some([
        // Pattern match should mean parsing always success
        cap.get(1)?.as_str().parse().ok()?,
        cap.get(3)?.as_str().parse().ok()?
    ])
}

/// Splits a Vec of points to 2 Vec of x and y
pub fn split_points(points: &Vec<[f64;2]>) -> (Vec<f64>, Vec<f64>) {
    let x_vals = points.iter().map(|point| point[0]).collect();
    let y_vals = points.iter().map(|point| point[1]).collect();

    (x_vals, y_vals)
}

pub fn _get_points_by_index(points: &Vec<String>) -> Result<(Vec<f64>, Vec<f64>), Box<dyn error::Error>> {
    let x_vals = (0..points.len()).map(|val| val as f64).collect();
    let y_vals = points.iter().map(|val| val.parse::<f64>()).collect::<Result<_, _>>()?;

    Ok((x_vals, y_vals))
}