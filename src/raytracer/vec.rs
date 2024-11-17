use super::types::Vector3;

extern crate nalgebra as na;

/// Reflects a vector `v` around a normal `n`.
pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - 2.0 * v.dot(&n) * n
}

/// Mirrors a vector `v` around a normal `n`.
pub fn mirror(v: Vector3, n: Vector3) -> Vector3 {
    2.0 * v.dot(&n) * n - v
}
