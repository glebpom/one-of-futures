#![doc(html_root_url = "https://docs.rs/one-of-futures/0.1.3")]
#![warn(missing_debug_implementations, rust_2018_idioms, unreachable_pub)]
#![deny(intra_doc_link_resolution_failure)]

//! This crate implements several custom future-aware `OneOf` types, which behaves
//! similarly to [`Either`](https://docs.rs/futures/0.3.1/futures/future/enum.Either.html) type,
//! but suitable for more than two variants.
//!
//! It also exposes `impl_one_of!` macro, which allow to generate custom `OneOf` type,
//! with the desired number and names of variants
//!
//! # Features
//!
//! `futures_03` enables support for futures 0.3 (core/std), enabled by default
//!
//! `sink` enables support fot futures 0.3 (core/std) 's `Sink`, enabled by default
//!
//! `futures_01` enables support for futures 0.1 under the futures_01 module
//!
//! [`html_root_url`]: https://rust-lang-nursery.github.io/api-guidelines/documentation.html#crate-sets-html_root_url-attribute-c-html-root

#[cfg(feature = "futures_01")]
pub mod futures_01;

#[cfg(feature = "futures_03")]
mod futures_03;
#[cfg(feature = "futures_03")]
pub use futures_03::*;
#[cfg(feature = "futures_03")]
pub use futures_core::{FusedFuture, FusedStream, Stream};

#[cfg(feature = "sink")]
pub use futures_sink::Sink;

#[cfg(feature = "tokio")]
pub use tokio::io::{AsyncRead, AsyncWrite};

#[cfg(test)]
mod tests {
    #[test]
    fn test_readme_deps() {
        version_sync::assert_markdown_deps_updated!("README.md");
    }

    #[test]
    fn test_html_root_url() {
        version_sync::assert_html_root_url_updated!("src/lib.rs");
    }
}
