extern crate rulinalg;

use rulinalg::matrix::Matrix;

pub struct Transformation {
    pub trans_mat : Matrix,
    pub trans_mat_inv: Matrix,
    pub trans_mat_transposed: Matrix,
    pub trans_mat_inv_transposed: Matrix,
};

pub impl Transformation {
    fn update(transformation: Transformation, matrix: Matrix) {

        transformation.trans_mat = matrix;

        transformation.trans_mat_inv = matrix
                                        .clone()
                                        .inverse()
                                        .expect("Matrix was not square when attempting inversion.");

        transformation.trans_mat_transposed = matrix
                                        .clone()
                                        .transpose();

        transformation.trans_mat_inv_transposed = matrix
                                        .clone()
                                        .inverse()
                                        .expect("Matrix was not square when attempting inversion.");
                                        .transpose();
    }
}
