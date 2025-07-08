use core::ops::Mul;

#[cfg(feature = "f64")]
use glam::{DMat2, DMat3, DMat4, DVec2, DVec3, DVec4};
use glam::{Mat2, Mat3, Mat3A, Mat4, Vec2, Vec3, Vec3A, Vec4};

// TODO: Implement optimized versions of the `inverse_or_zero` method.

/// An extension trait for matrices.
pub trait SquareMatExt {
    /// The type of the diagonal of the matrix.
    type Diagonal;

    /// Returns the diagonal of the matrix.
    #[must_use]
    fn diagonal(&self) -> Self::Diagonal;

    /// Returns the inverse of the matrix, or zero if the matrix is singular.
    #[must_use]
    fn inverse_or_zero(&self) -> Self;

    /// Returns `true` if the matrix is symmetric.
    #[must_use]
    fn is_symmetric(&self) -> bool;
}

impl SquareMatExt for Mat2 {
    type Diagonal = Vec2;

    #[inline]
    fn inverse_or_zero(&self) -> Self {
        let inverse = self.inverse();
        if inverse.is_finite() {
            inverse
        } else {
            Mat2::ZERO
        }
    }

    #[inline]
    fn diagonal(&self) -> Self::Diagonal {
        Vec2::new(self.x_axis.x, self.y_axis.y)
    }

    #[inline]
    fn is_symmetric(&self) -> bool {
        self.x_axis.y == self.y_axis.x
    }
}

#[cfg(feature = "f64")]
impl SquareMatExt for DMat2 {
    type Diagonal = DVec2;

    #[inline]
    fn inverse_or_zero(&self) -> Self {
        let inverse = self.inverse();
        if inverse.is_finite() {
            inverse
        } else {
            DMat2::ZERO
        }
    }

    #[inline]
    fn diagonal(&self) -> Self::Diagonal {
        DVec2::new(self.x_axis.x, self.y_axis.y)
    }

    #[inline]
    fn is_symmetric(&self) -> bool {
        self.x_axis.y == self.y_axis.x
    }
}

impl SquareMatExt for Mat3 {
    type Diagonal = Vec3;

    #[inline]
    fn inverse_or_zero(&self) -> Self {
        let tmp0 = self.y_axis.cross(self.z_axis);
        let tmp1 = self.z_axis.cross(self.x_axis);
        let tmp2 = self.x_axis.cross(self.y_axis);
        let det = self.z_axis.dot(tmp2);
        if det != 0.0 {
            let inv_det = Vec3::splat(det.recip());
            Self::from_cols(tmp0.mul(inv_det), tmp1.mul(inv_det), tmp2.mul(inv_det)).transpose()
        } else {
            Mat3::ZERO
        }
    }

    #[inline]
    fn diagonal(&self) -> Self::Diagonal {
        Vec3::new(self.x_axis.x, self.y_axis.y, self.z_axis.z)
    }

    #[inline]
    fn is_symmetric(&self) -> bool {
        self.x_axis.y == self.y_axis.x
            && self.x_axis.z == self.z_axis.x
            && self.y_axis.z == self.z_axis.y
    }
}

#[cfg(feature = "f64")]
impl SquareMatExt for DMat3 {
    type Diagonal = DVec3;

    #[inline]
    fn inverse_or_zero(&self) -> Self {
        let tmp0 = self.y_axis.cross(self.z_axis);
        let tmp1 = self.z_axis.cross(self.x_axis);
        let tmp2 = self.x_axis.cross(self.y_axis);
        let det = self.z_axis.dot(tmp2);
        if det != 0.0 {
            let inv_det = DVec3::splat(det.recip());
            Self::from_cols(tmp0.mul(inv_det), tmp1.mul(inv_det), tmp2.mul(inv_det)).transpose()
        } else {
            DMat3::ZERO
        }
    }

    #[inline]
    fn diagonal(&self) -> Self::Diagonal {
        DVec3::new(self.x_axis.x, self.y_axis.y, self.z_axis.z)
    }

    #[inline]
    fn is_symmetric(&self) -> bool {
        self.x_axis.y == self.y_axis.x
            && self.x_axis.z == self.z_axis.x
            && self.y_axis.z == self.z_axis.y
    }
}

impl SquareMatExt for Mat3A {
    type Diagonal = Vec3A;

    #[inline]
    fn inverse_or_zero(&self) -> Self {
        let tmp0 = self.y_axis.cross(self.z_axis);
        let tmp1 = self.z_axis.cross(self.x_axis);
        let tmp2 = self.x_axis.cross(self.y_axis);
        let det = self.z_axis.dot(tmp2);
        if det != 0.0 {
            let inv_det = Vec3A::splat(det.recip());
            Self::from_cols(tmp0.mul(inv_det), tmp1.mul(inv_det), tmp2.mul(inv_det)).transpose()
        } else {
            Mat3A::ZERO
        }
    }

    #[inline]
    fn diagonal(&self) -> Self::Diagonal {
        Vec3A::new(self.x_axis.x, self.y_axis.y, self.z_axis.z)
    }

    #[inline]
    fn is_symmetric(&self) -> bool {
        self.x_axis.y == self.y_axis.x
            && self.x_axis.z == self.z_axis.x
            && self.y_axis.z == self.z_axis.y
    }
}

impl SquareMatExt for Mat4 {
    type Diagonal = Vec4;

    #[inline]
    fn inverse_or_zero(&self) -> Self {
        let inverse = self.inverse();
        if inverse.is_finite() {
            inverse
        } else {
            Mat4::ZERO
        }
    }

    #[inline]
    fn is_symmetric(&self) -> bool {
        self.x_axis.y == self.y_axis.x
            && self.x_axis.z == self.z_axis.x
            && self.x_axis.w == self.w_axis.x
            && self.y_axis.z == self.z_axis.y
            && self.y_axis.w == self.w_axis.y
            && self.z_axis.w == self.w_axis.z
    }

    #[inline]
    fn diagonal(&self) -> Self::Diagonal {
        Vec4::new(self.x_axis.x, self.y_axis.y, self.z_axis.z, self.w_axis.w)
    }
}

#[cfg(feature = "f64")]
impl SquareMatExt for DMat4 {
    type Diagonal = DVec4;

    #[inline]
    fn inverse_or_zero(&self) -> Self {
        let inverse = self.inverse();
        if inverse.is_finite() {
            inverse
        } else {
            DMat4::ZERO
        }
    }

    #[inline]
    fn is_symmetric(&self) -> bool {
        self.x_axis.y == self.y_axis.x
            && self.x_axis.z == self.z_axis.x
            && self.x_axis.w == self.w_axis.x
            && self.y_axis.z == self.z_axis.y
            && self.y_axis.w == self.w_axis.y
            && self.z_axis.w == self.w_axis.z
    }

    #[inline]
    fn diagonal(&self) -> Self::Diagonal {
        DVec4::new(self.x_axis.x, self.y_axis.y, self.z_axis.z, self.w_axis.w)
    }
}
