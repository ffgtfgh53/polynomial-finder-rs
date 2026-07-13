use std::error;
use regex::Regex;

pub fn display_float(f: f64) -> String {
    let formatted = (f as f32).to_string();
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

pub fn display_point([x, y]: [f64;2]) -> String {
    format!("({}, {})", display_float(x), display_float(y))
}

pub fn get_points(input: &str) -> Option<[f64;2]> {
    let re = Regex::new(r"(-?\d+(\.\d+)?)[^\d\.]+(-?\d+(\.\d+)?)").unwrap(); // should always compile properly
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

#[allow(dead_code)]
pub mod coefficients {
    use crate::calculator::float_parser;

    pub fn insert_term(res: &mut String, exp: f64, n: usize) -> Result<(), String>{
        if exp.is_nan() {
            Err("No valid polynomial possible".to_string())
        } else if exp as f32 == 0.0 { 
            Ok(())
        } else {
            res.push_str(&format!(
                "{}{}", 
                format_exp(exp, !res.is_empty(), n==0),
                get_pow_x(n)
            ));
            Ok(())
        }
    }

    fn get_pow_x(n: usize) -> String {
        if n == 0 { "".to_string() }
        else if n == 1 { "x".to_string() }
        else { format!("x^{}", n) }
    }

    pub fn format_exp(exp: f64, add_plus_sign: bool, is_last_element: bool) -> String {
        let res = match exp {
            1.0 if !is_last_element => String::new(),
            -1.0 if !is_last_element => String::from("-"),
            exp => float_parser::display_float(exp)
        };

        if add_plus_sign && exp > 0.0 {
            "+".to_string() + &res.to_string()
        } else {
            res
        }
    }

}