//! [LAPACK] implementation of choice.
//!
//! Note that this package does not contain any functionality other than
//! compiling (if necessary) and linking to the chosen implementation. Bindings
//! are available in [`lapack-sys`][crate-lapack-sys], and wrappers are
//! available in [`lapack`][crate-lapack].
//!
//! ## Configuration
//!
//! The following implementations are available:
//!
//! * `accelerate`, which is the one in the [Accelerate] framework (macOS only),
//! * `netlib`, which is the reference one by [Netlib], and
//! * `openblas`, which is the one in [OpenBLAS].
//!
//! An implementation can be chosen as follows:
//!
//! ```toml
//! [dependencies]
//! lapack-src = { version = "0.1", features = ["accelerate"] }
//! lapack-src = { version = "0.1", features = ["netlib"] }
//! lapack-src = { version = "0.1", features = ["openblas"] }
//! ```
//!
//! [lapack]: https://en.wikipedia.org/wiki/LAPACK
//! [crate-lapack-sys]: https://crates.io/crates/lapack-sys
//! [crate-lapack]: https://crates.io/crates/lapack
//!
//! [accelerate]: https://developer.apple.com/reference/accelerate
//! [netlib]: http://www.netlib.org/
//! [openblas]: http://www.openblas.net/

#![no_std]

#[cfg(feature = "accelerate")]
extern crate accelerate_src as raw;

#[cfg(feature = "netlib")]
extern crate netlib_src as raw;

#[cfg(feature = "openblas")]
extern crate openblas_src as raw;
