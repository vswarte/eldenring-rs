use std::ops::{Mul, MulAssign};

use nalgebra::RowVector4;
use nalgebra_glm::{Mat4, Vec4};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Vector4(pub f32, pub f32, pub f32, pub f32);

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Matrix4X4(pub Vector4, pub Vector4, pub Vector4, pub Vector4);

impl From<Matrix4X4> for Mat4 {
    fn from(val: Matrix4X4) -> Self {
        Mat4::from_columns(&[
            Vec4::new(val.0.0, val.0.1, val.0.2, val.0.3),
            Vec4::new(val.1.0, val.1.1, val.1.2, val.1.3),
            Vec4::new(val.2.0, val.2.1, val.2.2, val.2.3),
            Vec4::new(val.3.0, val.3.1, val.3.2, val.3.3),
        ])
    }
}

impl From<Mat4> for Matrix4X4 {
    fn from(value: Mat4) -> Self {
        Self(
            Vector4(value[(0, 0)], value[(1, 0)], value[(2, 0)], value[(3, 0)]),
            Vector4(value[(0, 1)], value[(1, 1)], value[(2, 1)], value[(3, 1)]),
            Vector4(value[(0, 2)], value[(1, 2)], value[(2, 2)], value[(3, 2)]),
            Vector4(value[(0, 3)], value[(1, 3)], value[(2, 3)], value[(3, 3)]),
        )
    }
}

impl Mul<Mat4> for Matrix4X4 {
    type Output = Matrix4X4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        let lhs: Mat4 = self.clone().into();

        (lhs * rhs).into()
    }
}
