# Overpunch &emsp; [![Build Status]][actions] [![Latest Version]][crates.io]

[Build Status]: https://img.shields.io/endpoint.svg?url=https%3A%2F%2Factions-badge.atrox.dev%2Fcapitalrx%2Foverpunch%2Fbadge&style=flat

[actions]: https://actions-badge.atrox.dev/capitalrx/overpunch/goto

[Latest Version]: https://img.shields.io/crates/v/overpunch.svg

[crates.io]: https://crates.io/crates/overpunch

A [signed overpunch](https://en.wikipedia.org/wiki/Signed_overpunch) support library suitable for interacting
with [Decimal](https://docs.rs/rust_decimal/latest/rust_decimal/) values.

## Installing

```sh
$ cargo add overpunch
```

Alternatively, you can edit your `Cargo.toml` directly and run `cargo update`:

```toml
[dependencies]
overpunch = "0.2.0"
```

## Usage

To parse signed overpunch numbers:

```rust
use overpunch::{convert_from_signed_format, extract};
use rust_decimal::Decimal;

let number = convert_from_signed_format("2258{", "s9(7)v99").unwrap();
assert_eq!(number, Decimal::from_str_exact("225.8").unwrap());

let number = extract("2258{", 2).unwrap();
assert_eq!(number, Decimal::from_str_exact("225.8").unwrap());
```

To format values to signed overpunch:

```rust
use overpunch::{convert_to_signed_format, format};
use rust_decimal::Decimal;

let formatted = convert_to_signed_format(Decimal::from_str_exact("225.8").unwrap(), "s9(7)v99").unwrap();
assert_eq!(formatted, "2258{");

let formatted = format(Decimal::from_str_exact("225.8").unwrap(), 2).unwrap();
assert_eq!(formatted, "2258{");
```
