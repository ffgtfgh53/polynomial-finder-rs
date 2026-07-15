#![expect(clippy::pub_use, reason = "Re-exporting all structures and traits")]

mod circle;
mod points;

// Re-export structures at this root
pub use crate::structures::circle::Circle;
pub use crate::structures::points::PointsVector;

pub use crate::structures::points::CalulateResult;
pub use crate::structures::points::Split2DArray;

pub type Area = f64;