mod circle;
mod points;

pub type Area = f64;

// Re-export structures at this root
pub use crate::structures::circle::Circle;
pub use crate::structures::points::PointsVector;

pub use crate::structures::points::CalulateResult;
pub use crate::structures::points::Split2DArray;