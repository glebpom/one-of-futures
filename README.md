# one-of-futures
[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status](https://travis-ci.org/glebpom/one-of-futures.svg?branch=master)](https://travis-ci.org/glebpom/one-of-futures)

[crates-badge]: https://img.shields.io/crates/v/one-of-futures.svg
[crates-url]: https://crates.io/crates/one-of-futures
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE

## Overview
This crate implements several custom future-aware `OneOf` types, which behaves
similarly to [`Either`](https://docs.rs/futures/0.3.1/futures/future/enum.Either.html) type,
but suitable for more than two variants.

It also exposes `impl_one_of!` macro, which allows generating custom `OneOf` type,
with the desired number and names of variants

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
one-of-futures = "0.1"
```

## Example
```rust
use one_of_futures::impl_one_of;

impl_one_of!(MyEither;
  Left,
  Right
);

fn main() {
  let either = match 1 {
      0 => MyEither::Left(async { 1 }),
      _ => MyEither::Right(async { 2 }),
  };
}
```
