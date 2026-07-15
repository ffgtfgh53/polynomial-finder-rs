use std::iter::zip;

use itertools::Itertools as _;

use crate::calculator::float_parser;
use crate::structures::{
    Area, 
    CalulateResult, 
    PointsVector, 
    Split2DArray as _
};

impl TryFrom<&PointsVector> for Area {
    type Error = String;

    #[inline] fn try_from(value: &PointsVector) -> Result<Self, Self::Error> {
        if value.len() < 3 { 
            return Err("Need > 3 points to calculate area".to_owned());
        }

        solve_from_points(value.split_2d_array())
    }
}

impl CalulateResult for Area {
    /// Return the formatted result or formatted errors.
    /// 
    /// Points given in clockwise order will give A >= 0, and vice versa. Formula also applies to self-overlapping polygons.
    fn calc_from_points(points: &PointsVector) -> Result<String, String> {
        let solved = Area::try_from(points)?;
        Ok("A = ".to_owned() + &float_parser::display_float(solved)) 
    }
}

/// Reference: <https://en.wikipedia.org/wiki/Shoelace_formula#Triangle_formula>
fn solve_from_points((x_vals, y_vals): (Vec<f64>, Vec<f64>)) -> Result<f64, String>{
    if x_vals.len() != y_vals.len() {
        return Err("x_vals and y_vals have different lengths".to_owned())
    } 

    let mut sum = 0.;

    for ((x1, y1), (x2, y2)) 
        in zip(x_vals, y_vals).circular_tuple_windows() {
        sum += x1 * y2 - x2 * y1;
    }

    Ok(sum * 0.5)
}
