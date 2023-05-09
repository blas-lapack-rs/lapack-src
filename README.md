# lapack-src [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The packages provides a [LAPACK] source of choice.

## [Architecture]

## Configuration

The following implementations are available:

* `accelerate`, which is the one in the [Accelerate] framework (macOS only),
* `intel-mkl`, which is the one in [Intel MKL],
* `netlib`, which is the reference one by [Netlib], and
* `openblas`, which is the one in [OpenBLAS].

An implementation can be chosen as follows:

```toml
[dependencies]
lapack-src = { version = "0.8", features = ["accelerate"] }
lapack-src = { version = "0.8", features = ["intel-mkl"] }
lapack-src = { version = "0.8", features = ["netlib"] }
lapack-src = { version = "0.8", features = ["openblas"] }
r-src = { version = "0.8", features = ["r"] }
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[architecture]: https://blas-lapack-rs.github.io/architecture
[lapack]: https://en.wikipedia.org/wiki/LAPACK

[accelerate]: https://developer.apple.com/reference/accelerate
[intel mkl]: https://software.intel.com/en-us/mkl
[netlib]: http://www.netlib.org/
[openblas]: http://www.openblas.net/
[R]: https://cran.r-project.org

[build-img]: https://travis-ci.org/blas-lapack-rs/lapack-src.svg?branch=master
[build-url]: https://travis-ci.org/blas-lapack-rs/lapack-src
[documentation-img]: https://docs.rs/lapack-src/badge.svg
[documentation-url]: https://docs.rs/lapack-src
[package-img]: https://img.shields.io/crates/v/lapack-src.svg
[package-url]: https://crates.io/crates/lapack-src
