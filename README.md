# nostd

`nostd` is a lightweight Rust crate to simplify `no_std` development by bridging the gap with `std`.

It provides:

* Missing Prelude Types: Re-exports types include in the `std` prelude but missing from `core`, making `no_std` development smoother.
* IO Module Emulation: Re-exports [`core2`] providing a `std::io`-like API for `no_std` environments.

[`core2`]: https://docs.rs/crate/core2/latest

## Contributing

Contributions are welcome! If you find a bug, have a feature request, or wish to expand functionality, feel free to open an issue or submit a pull request.

## License

Licensed under either of:

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
