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
//! * `netlib`, which is the reference one by [Netlib],
//! * `openblas`, which is the one in [OpenBLAS], and
//! * `r`, which is the one in [R].
//!
//! An implementation can be chosen as follows:
//!
//! ```toml
//! [dependencies]
//! lapack-src = { version = "0.11", features = ["accelerate"] }
//! lapack-src = { version = "0.11", features = ["intel-mkl"] }
//! lapack-src = { version = "0.11", features = ["netlib"] }
//! lapack-src = { version = "0.11", features = ["openblas"] }
//! lapack-src = { version = "0.11", features = ["r"] }
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
    feature = "intel-mkl",
    feature = "intel-mkl-static-lp64-iomp",
    feature = "intel-mkl-static-lp64-seq",
    feature = "intel-mkl-static-ilp64-iomp",
    feature = "intel-mkl-static-ilp64-seq",
    feature = "intel-mkl-dynamic-lp64-iomp",
    feature = "intel-mkl-dynamic-lp64-seq",
    feature = "intel-mkl-dynamic-ilp64-iomp",
    feature = "intel-mkl-dynamic-ilp64-seq"
))]
extern crate intel_mkl_src as raw;

#[cfg(feature = "netlib")]
extern crate netlib_src as raw;

#[cfg(feature = "openblas")]
extern crate openblas_src as raw;

#[cfg(feature = "r")]
extern crate r_src as raw;
