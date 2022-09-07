pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

/// Calculate quotient, but take into account negative values so that they continue the cycle seamlessly.
/// e.g. (0, 4) -> 0, (-4, 4) -> -1, (-5, 4) -> -2
pub fn wrapping_quotient(dividend: i32, divisor: i32) -> i32 {
    let res = (if dividend < 0 { dividend + 1 } else { dividend }) / divisor;
    if dividend < 0 {
        res - 1
    } else {
        res
    }
}

/// Calculate remainder, but take into account negative values so that they continue the cycle seamlessly.
/// e.g. (0, 4) -> 0, (-4, 4) -> 0, (-5, 4) -> 3
pub fn wrapping_remainder(dividend: i32, divisor: i32) -> i32 {
    let res = dividend % divisor;
    if dividend < 0 {
        (divisor + res) % divisor
    } else {
        res
    }
}
