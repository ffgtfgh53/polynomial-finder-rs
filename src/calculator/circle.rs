use std::{fmt::Display, iter::zip};

use peroxide::fuga::{LinearAlgebra, MutFP, Shape::Col, choose_shorter_string, matrix};

use crate::calculator::float_parser;

struct Circle {
    midpoint: [f64;2],
    radius: f64,
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        result.push_str("(x");
        if self.midpoint[0] > 0. {
            let temp = choose_shorter_string(
                format!(" - {}", self.midpoint[0] as f32), 
                format!(" - {:.4}", self.midpoint[0] as f32)
            );
            result.push_str(&temp);
        } else if self.midpoint[0] < 0. {
            let temp = choose_shorter_string(
                format!(" + {}", -self.midpoint[0] as f32),
                format!(" + {:.4}", -self.midpoint[0] as f32)
            );
            result.push_str(&temp);
        };

        result.push_str(")^2 + (y");
        
        if self.midpoint[1] > 0. {
            let temp = choose_shorter_string(
                format!(" - {}", self.midpoint[1] as f32), 
                format!(" - {:.4}", self.midpoint[1] as f32)
            );
            result.push_str(&temp);
        } else if self.midpoint[1] < 0. {
            let temp = choose_shorter_string(
                format!(" + {}", -self.midpoint[1] as f32),
                format!(" + {:.4}", -self.midpoint[1] as f32)
            );
            result.push_str(&temp);
        };

        result.push_str(")^2 = ");

        let temp = choose_shorter_string(
            format!("{}", self.radius.powi(2) as f32), 
            format!("{:.4}", self.radius.powi(2) as f32)
        );
        result.push_str(&temp);

        write!(f, "{}", result)
    }
}

pub fn solve_by_points(points: &Vec<[f64; 2]>) -> Result<String, String> {
    let data = solve_from_points(float_parser::split_points(points))?;
    Ok(data.to_string())
}

/// Reference: https://planetcalc.com/8116/
fn solve_from_points((x_vals, y_vals): (Vec<f64>, Vec<f64>)) -> Result<Circle, String>{
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
        .map(|(x, y)| -(x.powi(2) + y.powi(2)))
        .collect();

    let b = matrix(b_data, 3, 1, Col);

    let v = (a.inv() * b).into_vec();

    let (x, y, c) = (-v[0], -v[1], v[2]);
    let r = (x.powi(2) + y.powi(2) - c).sqrt();

    if x.is_nan() || y.is_nan() || r.is_nan() {
        Err("Error when constructing circle (repeated or colinear points".to_string())
    } else {
        Ok(Circle{ 
            midpoint: [x, y], 
            radius: r 
        })
    }
}