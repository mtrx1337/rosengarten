extern crate rulinalg;

use crate::raytracer::intersection::Intersection;

pub trait Shape {
    fn intersect(&self) -> Intersection;
    fn new(&self) -> Box<dyn Shape>;
}
