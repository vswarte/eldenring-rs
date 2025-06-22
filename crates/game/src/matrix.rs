/// Defines some helper methods around dealing with math
use std::ops::{Add, Mul, MulAssign, Sub};

use nalgebra::RowVector4;
use nalgebra_glm::{Mat4, Vec3, Vec4};

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct FSVector4(pub f32, pub f32, pub f32, pub f32);

impl Sub<FSVector4> for FSVector4 {
    type Output = FSVector4;

    fn sub(self, rhs: FSVector4) -> Self::Output {
        FSVector4(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl Add<FSVector4> for FSVector4 {
    type Output = FSVector4;

    fn add(self, rhs: FSVector4) -> Self::Output {
        FSVector4(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FSVector3(pub f32, pub f32, pub f32);
impl From<FSVector3> for Vec3 {
    fn from(val: FSVector3) -> Self {
        Vec3::new(val.0, val.1, val.2)
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FSMatrix4x4(pub FSVector4, pub FSVector4, pub FSVector4, pub FSVector4);

impl From<FSMatrix4x4> for Mat4 {
    fn from(val: FSMatrix4x4) -> Self {
        Mat4::from_rows(&[
            RowVector4::new(val.0 .0, val.0 .1, val.0 .2, val.0 .3),
            RowVector4::new(val.1 .0, val.1 .1, val.1 .2, val.1 .3),
            RowVector4::new(val.2 .0, val.2 .1, val.2 .2, val.2 .3),
            RowVector4::new(val.3 .0, val.3 .1, val.3 .2, val.3 .3),
        ])
    }
}

impl From<Mat4> for FSMatrix4x4 {
    fn from(value: Mat4) -> Self {
        Self(
            FSVector4(value.m11, value.m12, value.m13, value.m14),
            FSVector4(value.m21, value.m22, value.m23, value.m24),
            FSVector4(value.m31, value.m32, value.m33, value.m34),
            FSVector4(value.m41, value.m42, value.m43, value.m44),
        )
    }
}

impl Mul<Mat4> for FSMatrix4x4 {
    type Output = FSMatrix4x4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        let lhs: Mat4 = self.clone().into();

        (lhs * rhs).into()
    }
}
