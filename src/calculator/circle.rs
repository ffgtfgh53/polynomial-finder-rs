use std::iter;

use peroxide::fuga::{
    LinearAlgebra, 
    MutFP, 
    Shape, 
    matrix
};

use crate::structures::{CalulateResult, Circle, PointsVector, Split2DArray};

impl TryFrom<&PointsVector> for Circle {
    type Error = String;

    fn try_from(value: &PointsVector) -> Result<Self, Self::Error> {
        let (points, to_check) = value
            .split_at_checked(3)
            .ok_or("Need > 3 points to calculate circle".to_string())?;

        let circle = solve_from_points(points.split_2d_array())?;

        for point in to_check {
            if !circle.check_point(*point) {
                Err("No circle that satisfies all points")?;
            }
        }

        Ok(circle)
    }
}

impl CalulateResult for Circle {
    fn calc_from_points(points: &PointsVector) -> Result<String, String> {
        Ok(Circle::try_from(points)?.to_string())
    }
}

#[allow(clippy::many_single_char_names)]
/// Reference: <https://planetcalc.com/8116/>
// a * v = b => v = a.inv() * b
fn solve_from_points((x_vals, y_vals): (Vec<f64>, Vec<f64>)) -> Result<Circle, String>{
    if x_vals.len() != y_vals.len() {
        Err("mismatch in length for x_vals and y_vals".to_string())?;
    }

    let mut a_data = Vec::with_capacity(9);

    a_data.append(&mut x_vals.clone());
    a_data.append(&mut y_vals.clone());
    a_data.append(&mut vec![0.5, 0.5, 0.5]);
    a_data.mut_map(|n| n * 2.0 );

    let a = matrix(a_data, 3, 3, Shape::Col);

    let b_data = iter::zip(x_vals, y_vals)
        .map(|(x, y)| -(x.powi(2) + y.powi(2)))
        .collect();

    let b = matrix(b_data, 3, 1, Shape::Col);

    let v = (a.inv() * b).into_vec();

    let (x, y, c) = (-v[0], -v[1], v[2]);
    let r = (x.powi(2) + y.powi(2) - c).sqrt();

    if x.is_nan() || y.is_nan() || r.is_nan() {
        Err("Error when constructing circle (repeated or colinear points?)".to_string())
    } else {
        Ok(Circle::new([x, y], r))
    }
}