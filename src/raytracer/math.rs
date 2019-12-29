extern crate rulinalg;

use rulinalg::vector::Vector;
use rulinalg::matrix::Matrix;
use std::vec::Vec;

mod vec3 {
    struct Vec3<T> {
        coords: mut [T, 3];
    }

    impl Vec3<T> {
        fn to_str(&self) {
            println!("Vec3: [{},{},{}]", self.coords.x, self.coords.y, self.coords.z);
        }
    }

    /* Adds the given vectors and returns a newly allocated direction vector */
    pub trait Direction<T> for Vec3<T> {
        pub fn add(&self, Direction v) -> Self {
            v: Direction = self.copy();
            for (i, coord) in v.coords.enumerate() {
                coord += v.coords[i];
            }
            v
        }

        /* Subtracts the first vector (self) from the second and returns a newly allocated
         * direction vector */
        pub fn subtract(&self, Direction v) -> Self {
            v: Direction = self.copy();
            for (i, coord) in v.coords.enumerate() {
                coord -= v.coords[i];
            }
            v
        }

        /* Multiplies the given vector with itself (self) and returns a newly allocated
         * direction vector */
        pub fn multiply(&self, Position v) -> Self {
            v: Self = self.copy();
            for (i, coord) in v.coords.enumerate() {
                coord *= v.coords[i];
            }
            v
        }
    }

    pub trait Position<T> for Vec3<T> {
        /* Adds the given vectors and returns a newly allocated position vector */
        pub fn add(&self, Position v) -> Self {
            v: Self = self.copy();
            for (i, coord) in v.coords.enumerate() {
                coord += v.coords[i];
            }
            v
        }

        /* Subtracts the first vector (self) from the second and returns a newly
         * allocated position vector */
        pub fn add(&self, Position v) -> Self {
            v: Self = self.copy();
            for (i, coord) in v.coords.enumerate() {
                coord -= v.coords[i];
            }
            v
        }

        /* Multiplies the given vector with itself (self) and returns a newly allocated
         * position vector */
        pub fn multiply(&self, Position v) -> Self {
            v: Self = self.copy();
            for (i, coord) in v.coords.enumerate() {
                coord *= v.coords[i];
            }
            v
        }
    }
}

fn transform_direction(Vector<Vec::with_capacity(16)>) -> Vector<Vec::with_capacity(16)> {
    // TODO
}
