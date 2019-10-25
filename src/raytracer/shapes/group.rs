extern crate rulinalg;
use rulinalg::vector::Vector;

use crate::raytracer::shapes::Shape;
use crate::raytracer::tranformation::Transformation;
use crate::raytracer::ray::Ray;

pub struct Group {
    shapes: Vec<Shape>,
    tranformation: Transformation,
}

pub fn intersect(ray: Ray) {
    // TODO
}
