use itertools::Itertools as _;

use peroxide::fuga::{
    LinearAlgebra as _, 
    Polynomial, 
    Scalable as _, 
    Shape, 
    matrix, 
    poly
};

use crate::structures::{
    CalulateResult, 
    PointsVector, 
    Split2DArray as _
};

impl TryFrom<&PointsVector> for Polynomial {
    type Error = String;

    #[inline] fn try_from(value: &PointsVector) -> Result<Self, Self::Error> {
        if value.is_empty() { 
            return Err("Need > 0 points to calculate".to_owned());
        }
        solve_from_points(value.split_2d_array())
    }
}

impl CalulateResult for Polynomial {
    fn calc_from_points(points: &PointsVector) -> Result<String, String> {
        Ok("f(x) = ".to_owned() + &Polynomial::try_from(points)?.to_string())
    }
}

/// Generate the polynomial of the equation that satisfies the points in (x, y)
/// 
/// Reference: <https://bueler.github.io/numerical/assets/slides/F24/polynonewt.pdf>
#[expect(clippy::cast_possible_truncation, reason = "We deliberately want to lose precision of imprecise floating point operations")]
fn solve_from_points((x_vals, y_vals): (Vec<f64>, Vec<f64>)) -> Result<Polynomial, String>{
    let n = x_vals.len();
    if n != y_vals.len() { 
        return Err("mismatch in length for x_vals and y_vals".to_owned());
    }

    let mut a = matrix([1].repeat(n), n, 1, Shape::Col);
    for i in 1..n {
        #[expect(clippy::cast_possible_wrap, reason = "x_vals.len unlikely to exceed i32::MAX")]
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
        .skip_while(|c| c.abs() < f64::from(f32::EPSILON))
        .map(|c| {
            if c.is_nan() { 
                Err("No suitable polynomial found".to_owned()) 
            } else { 
                Ok(f64::from(c as f32)) 
            }
        })
        .try_collect()?;

    Ok(poly(v))
}

