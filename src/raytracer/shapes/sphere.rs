extern crate rulinalg;

use rulinalg::vector::Vector;

use shape;
use crate::raytracer::intersection;
use crate::raytracer::materials::material::Material;

pub impl shape::Shape {
    pub fn new(&self, center: Vector, radius: f32, material: Material) -> Shape {
        &self.center = center;
        &self.radius = radius;
        &self.material = material;
    }

    pub fn intersect(ray: Ray) {
        // hit vector coordinates
        let a : f32 = ray.norm_dir.elemul(ray.norm_dir);
        // TODO
    }
}
