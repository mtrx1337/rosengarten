extern crate rulinalg;

use crate::raytracer::materials::material::Material;
use rulinalg::vector::Vector;

pub struct Intersection {
    pub dir_vec_multiplier: f32,
    pub hit_vec: Vector<f32>,
    pub norm_vec: Vector<f32>,
    pub material: Material,
}
