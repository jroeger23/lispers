use super::types::Vector3;

extern crate nalgebra as na;

pub fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - 2.0 * v.dot(&n) * n
}

pub fn rotate(v: &Vector3, axis: &Vector3, angle: f32) -> Vector3 {
    //let axis = na::Unit::new_normalize(axis);
    //let rot = na::Rotation3::from_axis_angle(&axis, angle);
    //(rot * v)
    todo!()
}
