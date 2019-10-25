use rulinalg::vector::Vector;
use crate::raytracer::ray::Ray;

pub struct MaterialProperties {
    pub albedo : Vector<f32>,
    pub emission : Vector<f32>,
    pub ray : Ray,
}
