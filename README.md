# lapack-src [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The packages provides a [LAPACK] source of choice.

## [Architecture]

## Configuration

The following implementations are available:

* `accelerate`, which is the one in the [Accelerate] framework (macOS only),
* `intel-mkl`, which is the one in [Intel MKL],
* `netlib`, which is the reference one by [Netlib],
* `openblas`, which is the one in [OpenBLAS], and
* `r`, which is the one in [R].


An implementation can be chosen as follows:

```toml
[dependencies]
lapack-src = { version = "0.12", features = ["accelerate"] }
lapack-src = { version = "0.12", features = ["intel-mkl"] }
lapack-src = { version = "0.12", features = ["netlib"] }
lapack-src = { version = "0.12", features = ["openblas"] }
lapack-src = { version = "0.12", features = ["r"] }
```

### Intel MKL Configuration

The `intel-mkl` feature will *statically* link the *sequential LP64* version of
the MKL library. To link other versions of the library, check the `intel-mkl-*-*-*`
features inside this crate, which are analogous to the feature flags of
the [`intel-mkl-src` crate] (https://crates.io/crates/intel-mkl-src).

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[architecture]: https://blas-lapack-rs.github.io/architecture
[lapack]: https://en.wikipedia.org/wiki/LAPACK

[accelerate]: https://developer.apple.com/reference/accelerate
[intel mkl]: https://software.intel.com/en-us/mkl
[netlib]: https://www.netlib.org/
[openblas]: https://www.openblas.net/
[r]: https://cran.r-project.org/

[build-img]: https://github.com/blas-lapack-rs/lapack-src/actions/workflows/build.yml/badge.svg
[build-url]: https://github.com/blas-lapack-rs/lapack-src/actions/workflows/build.yml/
[documentation-img]: https://docs.rs/lapack-src/badge.svg
[documentation-url]: https://docs.rs/lapack-src
[package-img]: https://img.shields.io/crates/v/lapack-src.svg
[package-url]: https://crates.io/crates/lapack-src
