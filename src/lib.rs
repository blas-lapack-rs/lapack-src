//! [LAPACK] source of choice.
//!
//! The usage of the package is explained [here][usage].
//!
//! ## Configuration
//!
//! The following implementations are available:
//!
//! * `accelerate`, which is the one in the [Accelerate] framework (macOS only),
//! * `intel-mkl`, which is the one in [Intel MKL],
//! * `netlib`, which is the reference one by [Netlib], and
//! * `openblas`, which is the one in [OpenBLAS].
//!
//! An implementation can be chosen as follows:
//!
//! ```toml
//! [dependencies]
//! lapack-src = { version = "0.6", features = ["accelerate"] }
//! lapack-src = { version = "0.6", features = ["intel-mkl"] }
//! lapack-src = { version = "0.6", features = ["netlib"] }
//! lapack-src = { version = "0.6", features = ["openblas"] }
//! ```
//!
//! [accelerate]: https://developer.apple.com/reference/accelerate
//! [intel mkl]: https://software.intel.com/en-us/mkl
//! [lapack]: https://en.wikipedia.org/wiki/LAPACK
//! [netlib]: http://www.netlib.org/
//! [openblas]: http://www.openblas.net/
//! [usage]: https://blas-lapack-rs.github.io/usage

#![no_std]

#[cfg(feature = "accelerate")]
extern crate accelerate_src as raw;

#[cfg(feature = "intel-mkl")]
extern crate intel_mkl_src as raw;

#[cfg(feature = "netlib")]
extern crate netlib_src as raw;

#[cfg(feature = "openblas")]
extern crate openblas_src as raw;
