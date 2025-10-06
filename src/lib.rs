//! [LAPACK] source of choice.
//!
//! ## [Architecture]
//!
//! ## Configuration
//!
//! The following implementations are available:
//!
//! * `accelerate`, which is the one in the [Accelerate] framework (macOS only),
//! * `intel-mkl-*`, which is the one in [Intel MKL], where
//!   * `intel-mkl-dynamic-parallel` dynamically links the parallel backend of MKL
//!   * `intel-mkl-dynamic-sequential` dynamically links the sequential backend of MKL
//!   * `intel-mkl-static-parallel` statically links the parallel backend of MKL
//!   * `intel-mkl-static-sequential` statically links the sequential backend of MKL
//! * `netlib`, which is the reference one by [Netlib],
//! * `openblas`, which is the one in [OpenBLAS], and
//! * `r`, which is the one in [R].
//!
//! An implementation can be chosen as follows:
//!
//! ```toml
//! [dependencies]
//! lapack-src = { version = "0.12", features = ["accelerate"] }
//! lapack-src = { version = "0.12", features = ["intel-mkl"] }
//! lapack-src = { version = "0.12", features = ["netlib"] }
//! lapack-src = { version = "0.12", features = ["openblas"] }
//! lapack-src = { version = "0.12", features = ["r"] }
//! ```
//!
//! [architecture]: https://blas-lapack-rs.github.io/architecture
//! [lapack]: https://en.wikipedia.org/wiki/LAPACK
//!
//! [accelerate]: https://developer.apple.com/reference/accelerate
//! [intel mkl]: https://software.intel.com/en-us/mkl
//! [netlib]: https://www.netlib.org/
//! [openblas]: https://www.openblas.net/
//! [r]: https://cran.r-project.org/

#![no_std]

#[cfg(feature = "accelerate")]
extern crate accelerate_src as raw;

#[cfg(any(
    feature = "intel-mkl-dynamic-parallel",
    feature = "intel-mkl-dynamic-sequential",
    feature = "intel-mkl-static-parallel",
    feature = "intel-mkl-static-sequential",
))]
extern crate intel_mkl_src as raw;

#[cfg(feature = "netlib")]
extern crate netlib_src as raw;

#[cfg(feature = "openblas")]
extern crate openblas_src as raw;

#[cfg(feature = "r")]
extern crate r_src as raw;
