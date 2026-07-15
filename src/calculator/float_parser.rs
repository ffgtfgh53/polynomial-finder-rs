#![allow(clippy::cast_possible_truncation)]

use peroxide::fuga::choose_shorter_string;
use regex::Regex;

pub fn display_float(f: f64) -> String {
    choose_shorter_string(
        format!("{}", f as f32), 
        format!("{:.4}", f as f32)
    )
}

pub fn display_point([x, y]: [f64;2]) -> String {
    format!("({}, {})", display_float(x), display_float(y))
}

pub fn get_points(input: &str) -> Option<[f64;2]> {
    let re = Regex::new(r"(-?\d+(\.\d+)?)[^\d\.]+?(-?\d+(\.\d+)?)").unwrap(); // should always compile properly
    let cap = re.captures(input)?;

    Some([
        // Pattern match should mean parsing always success
        cap.get(1)?.as_str().parse().ok()?,
        cap.get(3)?.as_str().parse().ok()?
    ])
}