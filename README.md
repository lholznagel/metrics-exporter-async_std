# metrics-exporter-http-async_std

[![conduct-badge][]][conduct]
[![release-badge][]][crate]
[![docs-badge][]][docs]
[![license-badge][]](#license)

[conduct-badge]: https://img.shields.io/badge/%E2%9D%A4-code%20of%20conduct-blue.svg
[license-badge]: https://img.shields.io/badge/license-MIT-blue
[release-badge]: https://img.shields.io/crates/v/metrics-exporter-http-async_std.svg
[docs-badge]: https://docs.rs/metrics-exporter-http-async_std/badge.svg
[conduct]: https://github.com/lholznagel/metrics-exporter-http-async_std/blob/master/CODE_OF_CONDUCT.md
[crate]: https://crates.io/crates/metrics-exporter-http-async_std
[docs]: https://docs.rs/metrics-exporter-http-async_std

__metrics-exporter-http-async_std__ is a metrics-core compatible exporter for serving metrics over HTTP using async_std.

This crate is a drop in replacement for [metrics-exporter-http](https://github.com/metrics-rs/metrics/tree/master/metrics-exporter-http).
The main difference is that `metrics-exporter-http` uses hyper for serving.
This implementation uses [async_std](https://github.com/async-rs/async-std) for serving.

For more information see [metrics](https://github.com/metrics-rs/metrics).

## code of conduct

**NOTE**: All conversations and contributions to this project shall adhere to the [Code of Conduct][conduct].