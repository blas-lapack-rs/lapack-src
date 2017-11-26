# lapack-src [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The packages provides a [LAPACK] source of choice.

The usage of the package is explained [here][usage].

## Configuration

The following implementations are available:

* `accelerate`, which is the one in the [Accelerate] framework (macOS only),
* `netlib`, which is the reference one by [Netlib], and
* `openblas`, which is the one in [OpenBLAS].

An implementation can be chosen as follows:

```toml
[dependencies]
lapack-src = { version = "0.1", features = ["accelerate"] }
lapack-src = { version = "0.1", features = ["netlib"] }
lapack-src = { version = "0.1", features = ["openblas"] }
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[accelerate]: https://developer.apple.com/reference/accelerate
[lapack]: https://en.wikipedia.org/wiki/LAPACK
[netlib]: http://www.netlib.org/
[openblas]: http://www.openblas.net/
[usage]: https://blas-lapack-rs.github.io/usage

[build-img]: https://travis-ci.org/blas-lapack-rs/lapack-src.svg?branch=master
[build-url]: https://travis-ci.org/blas-lapack-rs/lapack-src
[documentation-img]: https://docs.rs/lapack-src/badge.svg
[documentation-url]: https://docs.rs/lapack-src
[package-img]: https://img.shields.io/crates/v/lapack-src.svg
[package-url]: https://crates.io/crates/lapack-src
