# reverb

[![CI](https://github.com/Celeo/reverb/workflows/CI/badge.svg?branch=master)](https://github.com/celeo/reverb/actions?query=workflow%3ACI)

Simple websocket broadcast server with rooms.

## Installing

1. Download a binary from the [GitHub releases page](https://github.com/Celeo/reverb)

## Using

1. Download the binary (see above)
1. Run, optionally passing the URL & port to bind to
1. Connect with a websocket implementation (browser, or something like [websocat](https://crates.io/crates/websocat))

***Rooms are in-dev***

## Developing

### Building

### Requirements

* Git
* A recent version of [Rust](https://www.rust-lang.org/tools/install)

### Steps

```sh
git clone https://github.com/Celeo/reverb
cd reverb
cargo build
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

## Contributing

Please feel free to contribute. Please open an issue first (or comment on an existing one) so that I know that you want to add/change something.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
