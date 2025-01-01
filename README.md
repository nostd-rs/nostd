# nostd

[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/nostd-rs/nostd/ci.yml?event=push)](https://github.com/nostd-rs/nostd)
[![Crates.io Version](https://img.shields.io/crates/v/nostd)](https://crates.io/crates/nostd)
[![GitHub License](https://img.shields.io/badge/license-MIT%2FApache2-blue)](#LICENSE)

`nostd` provides essential `std`-like types in `no_std` environments.

This crate re-exports types from the `alloc` and `core` crates under the
familiar `std` path while also offering alternative implementations for
types not available in those crates.

`nostd` aims to help port code written for `std` to `no_std` with minimal
changes, often requiring only a replacement of `std::` with `nostd::`.

However, please note that a successful build does not guarantee that the
code will work as expected. Exercise caution and thoroughly test your
application before using it in production.

## Features

- `std::prelude` emulation
```rs
use nostd::prelude::*;
// Now you can access `Box`, `String`, `Vec`, etc.
```
- `std::io` emulation by [`core2`] (feature: `io`)
- `std::collections::hash_(map|set)` emulation by [`hashbrown`]
  (feature: `hashbrown`)

[`core2`]: https://crates.io/crates/core2
[`hashbrown`]: https://crates.io/crates/hashbrown

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
nostd = "0.1"
```

By default, `nostd` is intended to be used in `no_std` environment. To use
it in `std` environment, enable the `std` feature:

```toml
[features]
default = ["std"]
std = ["nostd/std"]
```

## Contributing

Contributions are welcome! If you find a bug, have a feature request, or wish
to expand functionality, feel free to open an issue or submit a pull request.

## License

Licensed under either of:

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
