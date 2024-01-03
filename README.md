This repository demonstatetes the lack of support of `#[cfg(not(feature = "foo"))]` inside [`cxx`](https://github.com/dtolnay/cxx)'s `bridge` macro.

It simulates a situation where a C++ library changed its API in a new version removing some variants from a enum.
In this case it would be useful to have a feature flag to support both versions of the library.

Reproduce the issue by running `cargo build --features=v2`.