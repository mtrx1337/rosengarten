extern crate rulinalg;
extern crate arrayvec;

//use crate::raytracer::ray::Ray;
//use crate::raytracer::shapes::shape::Shape;
use crate::raytracer::transformation::Transformation;
//use crate::raytracer::intersection::Intersection;

pub struct Group<'s, Shape> {
    shapes: &'s[Shape; 25],
    tranformation: Transformation,
}

//impl Shape for Group<'_, Shape> {
//    fn intersect(&self, ray: Ray) -> Intersection {
//        let ray_dir = &self.transformation.trans_mat_inv * ray.norm_dir;
//        let ray_origin = &self.transformation.trans_mat_inv * ray.origin;
//        let dist;
//        let intersect_vec;
//        let intersect_material;
//
//        // TODO
//
//        let intersect = Intersection {
//            distance: intersect_dist,
//            hit_vec: intersect_vec,
//            norm_vec: intersect_norm_vec,
//            material: intersect_material
//        };
//        return intersect;
//    }
//}
