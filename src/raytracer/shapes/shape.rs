extern crate rulinalg;

use rulinalg::vector::Vector;
use crate::raytracer::materials::material::Material;
use crate::raytracer::ray::Ray;

pub struct Shape {
    intersect: Intersection,
    center: Vector,
    radius: f32,
    material: Material,

    pub fn intersect(ray: Ray),
}
