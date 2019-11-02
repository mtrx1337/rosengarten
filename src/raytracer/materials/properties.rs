use crate::raytracer::ray::Ray;
use rulinalg::vector::Vector;

pub struct MaterialProperties {
    pub albedo: Vector<f32>,
    pub emission: Vector<f32>,
    pub ray: Ray,
}
