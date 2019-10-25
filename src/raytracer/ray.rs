use rulinalg::vector::Vector;

pub struct Ray {
    pub origin: Vector<f32>,
    pub norm_dir: Vector<f32>,
    pub min : f32,
    pub max : f32,
}

pub fn point_at(ray: Ray, t: f32) -> Vector<f32> {
    return (ray.norm_dir * t) + ray.origin;
}
