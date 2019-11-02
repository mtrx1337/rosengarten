extern crate rulinalg;

use rulinalg::vector::Vector;
use crate::raytracer::ray::Ray;
use crate::raytracer::shapes::shape::Shape;
use crate::raytracer::intersection::Intersection;
use crate::raytracer::materials::material::Material;

struct Sphere {
    intersect: Intersection,
    center: Vector<f32>,
    radius: f32,
    material: Material,
}

impl Shape for Sphere {
    fn new(&self, center: Vector<f32>, radius: f32, material: Material) -> Shape {
        &self.center = center;
        &self.radius = radius;
        &self.material = material;
    }

    fn intersect(ray: Ray) {
        // hit vector coordinates
        let a: f32 = ray.norm_dir.elemul(ray.norm_dir);
        // TODO
    }
}
