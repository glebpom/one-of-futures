# one-of-futures
[![Build Status](https://travis-ci.org/glebpom/one-of-futures.svg?branch=master)](https://travis-ci.org/glebpom/one-of-futures)

This crate implements several custom future-aware `OneOf` types, which behaves
similarly to [`Either`](https://docs.rs/futures/0.3.1/futures/future/enum.Either.html) type,
but suitable for more than two variants.

It also exposes `impl_one_of!` macro, which allows generating custom `OneOf` type,
with the desired number and names of variants
