//! [Symmetric matrix] types and utilities.
//!
//! [Symmetric matrix]: https://en.wikipedia.org/wiki/Symmetric_matrix

mod symmetric_mat2;
mod symmetric_mat3;
mod symmetric_mat6;

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
#[cfg(feature = "f64")]
pub use symmetric_mat6::DSymmetricMat6;
#[cfg(feature = "f32")]
pub use symmetric_mat6::SymmetricMat6;
