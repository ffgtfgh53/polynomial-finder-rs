use std::fmt;

use peroxide::fuga::choose_shorter_string;

#[derive(Debug, Clone, Default)]
pub struct Circle {
    pub midpoint: [f64;2],
    pub radius: f64,
}

/// Similar to that of `peroxide::fuga::Polynomial`
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl Circle {
    pub fn new(midpoint: [f64;2], radius: f64) -> Self {
        Self { midpoint, radius }
    }

    /// Return true if the point lies on the circle
    pub fn check_point(&self, [x, y]: [f64;2]) -> bool {
        let [x1, y1] = self.midpoint;
        // Check if substitution matches to +- 3 * EPSILON
        (
            (x - x1).powi(2) 
            + (y - y1).powi(2) 
            - self.radius.powi(2)
        ).abs() < self.radius * 0.001 // Uncertainty of 0.1% (or 0.001% because of the ^2?)
    }
}