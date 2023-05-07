//
// EPITECH PROJECT, 2023
// transformationbuilder
// File description:
// FreeKOSOVO
//

use crate::matrix::Matrix;

pub struct TransformationBuilder {
    pub matrix: Matrix,
}

impl TransformationBuilder {
    pub fn new() -> TransformationBuilder {
        TransformationBuilder {
            matrix: Matrix::identity(4),
        }
    }

    pub fn translation(mut self, x: f64, y: f64, z: f64, w: f64) -> TransformationBuilder {
        let mut matrix = Matrix::identity(4);
        matrix[(0, 3)] = x;
        matrix[(1, 3)] = y;
        matrix[(2, 3)] = z;
        matrix[(3, 3)] = w;
        self.matrix *= matrix;
        self
    }

    pub fn scale(mut self, x: f64, y: f64, z: f64) -> TransformationBuilder {
        let mut matrix = Matrix::identity(4);
        matrix[(0, 0)] = x;
        matrix[(1, 1)] = y;
        matrix[(2, 2)] = z;
        self.matrix *= matrix;
        self
    }

    pub fn rotation_x(mut self, radians: f64) -> TransformationBuilder {
        let mut matrix = Matrix::identity(4);
        matrix[(1, 1)] = radians.cos();
        matrix[(1, 2)] = -radians.sin();
        matrix[(2, 1)] = radians.sin();
        matrix[(2, 2)] = radians.cos();
        self.matrix *= matrix;
        self
    }

    pub fn rotation_y(mut self, radians: f64) -> TransformationBuilder {
        let mut matrix = Matrix::identity(4);
        matrix[(0, 0)] = radians.cos();
        matrix[(0, 2)] = radians.sin();
        matrix[(2, 0)] = -radians.sin();
        matrix[(2, 2)] = radians.cos();
        self.matrix *= matrix;
        self
    }

    pub fn rotation_z(mut self, radians: f64) -> TransformationBuilder {
        let mut matrix = Matrix::identity(4);
        matrix[(0, 0)] = radians.cos();
        matrix[(0, 1)] = -radians.sin();
        matrix[(1, 0)] = radians.sin();
        matrix[(1, 1)] = radians.cos();
        self.matrix *= matrix;
        self
    }

    pub fn get_matrix(self) -> Matrix {
        self.matrix 
    }
}
