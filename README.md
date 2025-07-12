# `glam_mat_extensions`

Matrix types and utilities for [`glam`].

[`glam`]: https://docs.rs/glam/latest/glam/

## Features

- `SquareMatExt` extension trait with useful helpers like `is_symmetric`, `inverse_or_zero`, and `diagonal`
- Rectangular matrices
  - [x] 2x3 matrices: `Mat23`, `DMat23`
  - [x] 3x2 matrices: `Mat32`, `DMat32`
- Symmetric matrices
  - [x] Symmetric 2x2 matrices: `SymmetricMat2`, `DSymmetricMat2`
  - [x] Symmetric 3x3 matrices: `SymmetricMat3`, `DSymmetricMat3`
  - [ ] Symmetric 4x4 matrices: `SymmetricMat4`, `DSymmetricMat4`
  - [ ] Symmetric 5x5 matrices: `SymmetricMat5`, `DSymmetricMat5`
  - [x] Symmetric 6x6 matrices: `SymmetricMat6`, `DSymmetricMat6`
- Eigen decompositions of symmetric matrices
  - [x] 2D: `SymmetricEigen2`
  - [x] 3D: `SymmetricEigen3`

## License

`glam_mat_extensions` is free and open source. All code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](/LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.
