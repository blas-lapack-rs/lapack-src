//! [LAPACK] source of choice.
//!
//! ## [Architecture]
//!
//! ## Configuration
//!
//! The following implementations are available:
//!
//! * `accelerate`, which is the one in the [Accelerate] framework (macOS only),
//! * `intel-mkl`, which is the one in [Intel MKL],
//! * `netlib`, which is the reference one by [Netlib], and
//! * `openblas`, which is the one in [OpenBLAS].
//! * `r`, which is the one in [R].
//!
//! An implementation can be chosen as follows:
//!
//! ```toml
//! [dependencies]
//! lapack-src = { version = "0.8", features = ["accelerate"] }
//! lapack-src = { version = "0.8", features = ["intel-mkl"] }
//! lapack-src = { version = "0.8", features = ["netlib"] }
//! lapack-src = { version = "0.8", features = ["openblas"] }
//! lapack-src = { version = "0.8", features = ["r"] }
//! ```
//!
//! [architecture]: https://blas-lapack-rs.github.io/architecture
//! [lapack]: https://en.wikipedia.org/wiki/LAPACK
//!
//! [accelerate]: https://developer.apple.com/reference/accelerate
//! [intel mkl]: https://software.intel.com/en-us/mkl
//! [netlib]: http://www.netlib.org/
//! [openblas]: http://www.openblas.net/
//! [R]: https://cran.r-project.org

#![no_std]

#[cfg(feature = "accelerate")]
extern crate accelerate_src as raw;

#[cfg(feature = "intel-mkl")]
extern crate intel_mkl_src as raw;

#[cfg(feature = "netlib")]
extern crate netlib_src as raw;

#[cfg(feature = "openblas")]
extern crate openblas_src as raw;

#[cfg(feature = "r")]
extern crate r_src as raw;
