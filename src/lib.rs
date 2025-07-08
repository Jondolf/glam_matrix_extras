//! Matrix types and utilities for [`glam`].

#![warn(missing_docs)]

mod mat_ext;
mod symmetric_mat2;
mod symmetric_mat3;

pub use mat_ext::SquareMatExt;
#[cfg(feature = "f64")]
pub use symmetric_mat2::DSymmetricMat2;
pub use symmetric_mat2::Mat2Ext;
#[cfg(feature = "f32")]
pub use symmetric_mat2::SymmetricMat2;
#[cfg(feature = "f64")]
pub use symmetric_mat3::DSymmetricMat3;
pub use symmetric_mat3::Mat3Ext;
#[cfg(feature = "f32")]
pub use symmetric_mat3::SymmetricMat3;

/// An error that can occur when converting matrices to other representations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatConversionError {
    /// Tried to convert a matrix to a symmetric matrix type, but the matrix is not symmetric.
    Asymmetric,
}

impl core::fmt::Display for MatConversionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MatConversionError::Asymmetric => write!(f, "Matrix is not symmetric"),
        }
    }
}
