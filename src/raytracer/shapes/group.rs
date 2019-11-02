extern crate rulinalg;
extern crate arrayvec;
use rulinalg::vector::Vector;

use crate::raytracer::ray::Ray;
use crate::raytracer::shapes::shape::Shape;
use crate::raytracer::transformation::Transformation;
use crate::raytracer::intersection::Intersection;

pub struct Group<'a, Shape> {
    shapes: &'a[Shape; 25],
    tranformation: Transformation,
}

impl Shape for Group<'_, Shape> {
    fn intersect(&self, ray: Ray) -> Intersection {
        let ray_dir = transformation.trans_mat_inv * ray.norm_dir;
        let ray_origin = transformation.trans_mat_inv * ray.origin;
    }
}
