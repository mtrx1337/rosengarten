extern crate rulinalg;

use rulinalg::matrix::Matrix;

pub struct Transformation {
    pub trans_mat : Matrix<f32>,
    pub trans_mat_inv: Matrix<f32>,
    pub trans_mat_transposed: Matrix<f32>,
    pub trans_mat_inv_transposed: Matrix<f32>,
}

//impl Transformation {
//    fn new(&self, matrix: Matrix<f32>) {
//
//        &self.trans_mat = matrix;
//
//        &self.trans_mat_inv = matrix
//                                        .clone()
//                                        .inverse()
//                                        .expect("Matrix was not square when attempting inversion.");
//
//        &self.trans_mat_transposed = matrix
//                                        .clone()
//                                        .transpose();
//
//        &self.trans_mat_inv_transposed = matrix
//                                        .clone()
//                                        .inverse()
//                                        .expect("Matrix was not square when attempting inversion.")
//                                        .transpose();
//    }
//}
