use std::ops::{Deref, DerefMut};

pub trait Split2DArray {
    /// Split an array of [x, y] to an array of x and an array of y
    fn split_2d_array(&self) -> (Vec<f64>, Vec<f64>);
}

impl Split2DArray for [[f64; 2]] {
    fn split_2d_array(&self) -> (Vec<f64>, Vec<f64>) {
        (
            self.iter().map(|row| row[0]).collect(),
            self.iter().map(|row| row[1]).collect()
        )
    }
}


pub trait CalulateResult {
    /// Return the formatted result or formatted errors
    fn calc_from_points(points: &PointsVector) -> Result<String, String>;
}


#[derive(Debug, Default, Clone)]
pub struct PointsVector(Vec<[f64; 2]>);

// All methods for Vec available directly on PointsVector
impl Deref for PointsVector {
    type Target = Vec<[f64; 2]>;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PointsVector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}