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
//! lapack-src = { version = "0.12", features = ["accelerate"] }
//! lapack-src = { version = "0.12", features = ["intel-mkl"] }
//! lapack-src = { version = "0.12", features = ["netlib"] }
//! lapack-src = { version = "0.12", features = ["openblas"] }
//! lapack-src = { version = "0.12", features = ["r"] }
//! ```
//! ### Configuring MKL
//!
//! When the `intel-mkl` feature is selected, then the parallel version of
//! MKL using OpenMP is _statically_ linked. To link the sequential version
//! use the `intel-mkl-seq` feature. In both cases, the
//! [LP64 interface](https://www.intel.com/content/www/us/en/docs/onemkl/developer-guide-linux/2023-0/using-the-ilp64-interface-vs-lp64-interface.html)
//! is linked. If other linkage options for MKL are desired, omit `lapack-src`
//! as a dependency and use the [`intel-mkl-src`](https://crates.io/crates/intel-mkl-src)
//! crate directly with the appropriate feature flags.
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

#[cfg(any(feature = "intel-mkl", feature = "intel-mkl-seq"))]
extern crate intel_mkl_src as raw;

#[cfg(feature = "netlib")]
extern crate netlib_src as raw;

#[cfg(feature = "openblas")]
extern crate openblas_src as raw;

#[cfg(feature = "r")]
extern crate r_src as raw;
