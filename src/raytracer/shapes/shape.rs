extern crate rulinalg;

use crate::raytracer::materials::material::Material;
use crate::raytracer::ray::Ray;
use rulinalg::vector::Vector;

pub trait Shape {
    fn intersect(&self);
    fn new(&self);
}

pub trait Intersection {
    fn intersect(&self, ray: Ray) -> Intersection;
}
