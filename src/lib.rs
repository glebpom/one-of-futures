#![doc(html_root_url = "https://docs.rs/one-of-futures/0.1.0")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![deny(intra_doc_link_resolution_failure)]

//! This crate implements several custom future-aware `OneOf` types, which behaves
//! similarly to [`Either`](https://docs.rs/futures/0.3.1/futures/future/enum.Either.html) type,
//! but suitable for more than two variants.
//!
//! It also exposes `impl_one_of!` macro, which allow to generate custom `OneOf` type,
//! with the desired number and names of variants
//! [`html_root_url`]: https://rust-lang-nursery.github.io/api-guidelines/documentation.html#crate-sets-html_root_url-attribute-c-html-root

/// Macro to implement custom `OneOf` type.
///
/// Example:
/// ```rust
/// use one_of_futures::impl_one_of;
///
/// impl_one_of!(MyEither;
///   Left,
///   Right
/// );
///
/// fn main() {
///   let either = match 1 {
///       0 => MyEither::Left(async { 1 }),
///       _ => MyEither::Right(async { 2 }),
///   };
/// }
/// ```
#[macro_export]
macro_rules! impl_one_of (
    ($enum_name:ident ; $head_variant:ident, $($tail_variants:ident),*) => {
        /// Combines multiple different futures, streams, or sinks having the
        /// same associated types into a single type.
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum $enum_name<$head_variant, $( $tail_variants ),*> {
            $head_variant($head_variant),
            $( $tail_variants($tail_variants) ),*
        }

        impl<$head_variant, $($tail_variants),*> ::core::future::Future for
            $enum_name<$head_variant, $($tail_variants ),*>
            where
                $head_variant: ::core::future::Future,
                $( $tail_variants: ::core::future::Future<Output=$head_variant::Output> ),* {

            type Output = $head_variant::Output;

            fn poll(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>) ->::core::task::Poll<Self::Output> {
                unsafe {
                    match self.get_unchecked_mut() {
                        $enum_name::$head_variant(x) => ::core::pin::Pin::new_unchecked(x).poll(cx),
                        $(
                            $enum_name::$tail_variants(x) => ::core::pin::Pin::new_unchecked(x).poll(cx),
                        )*
                    }
                }
            }
        }

        impl<$head_variant, $($tail_variants),*> ::futures_core::FusedFuture for
            $enum_name<$head_variant, $($tail_variants ),*>
            where
                $head_variant: ::futures_core::FusedFuture,
                $( $tail_variants: ::futures_core::FusedFuture<Output=$head_variant::Output> ),* {

            fn is_terminated(&self) -> bool {
                match self {
                    $enum_name::$head_variant(x) => x.is_terminated(),
                    $(
                        $enum_name::$tail_variants(x) => x.is_terminated(),
                    )*
                }
            }
        }

         impl<$head_variant, $($tail_variants),*> ::futures_core::Stream for
            $enum_name<$head_variant, $($tail_variants ),*>
            where
                $head_variant: ::futures_core::Stream,
                $( $tail_variants: ::futures_core::Stream<Item=$head_variant::Item> ),* {

            type Item = $head_variant::Item;

            fn poll_next(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>) ->::core::task::Poll<Option<$head_variant::Item>> {
                unsafe {
                    match self.get_unchecked_mut() {
                        $enum_name::$head_variant(x) => ::core::pin::Pin::new_unchecked(x).poll_next(cx),
                        $(
                            $enum_name::$tail_variants(x) => ::core::pin::Pin::new_unchecked(x).poll_next(cx),
                        )*
                    }
                }
            }
        }

        impl<$head_variant, $($tail_variants),*> ::futures_core::FusedStream for
            $enum_name<$head_variant, $( $tail_variants ),*>
            where
                $head_variant: ::futures_core::FusedStream,
                $( $tail_variants: ::futures_core::FusedStream<Item=$head_variant::Item> ),* {

            fn is_terminated(&self) -> bool {
                match self {
                    $enum_name::$head_variant(x) => x.is_terminated(),
                    $(
                        $enum_name::$tail_variants(x) => x.is_terminated(),
                    )*
                }
            }
        }

        #[cfg(feature = "sink")]
        impl<Item, $head_variant, $($tail_variants),*> futures_sink::Sink<Item> for
            $enum_name<$head_variant, $( $tail_variants ),*>
            where
                $head_variant: futures_sink::Sink<Item>,
                $( $tail_variants: futures_sink::Sink<Item, Error=$head_variant::Error> ),* {
            type Error = $head_variant::Error;

            fn poll_ready(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>) ->::core::task::Poll<Result<(), Self::Error>> {
                unsafe {
                    match self.get_unchecked_mut() {
                        $enum_name::$head_variant(x) => ::core::pin::Pin::new_unchecked(x).poll_ready(cx),
                        $(
                            $enum_name::$tail_variants(x) => ::core::pin::Pin::new_unchecked(x).poll_ready(cx),
                        )*
                    }
                }
            }

            fn start_send(self: ::core::pin::Pin<&mut Self>, item: Item) -> Result<(), Self::Error> {
                unsafe {
                    match self.get_unchecked_mut() {
                        $enum_name::$head_variant(x) => ::core::pin::Pin::new_unchecked(x).start_send(item),
                        $(
                            $enum_name::$tail_variants(x) => ::core::pin::Pin::new_unchecked(x).start_send(item),
                        )*
                    }
                }
            }

            fn poll_flush(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>) ->::core::task::Poll<Result<(), Self::Error>> {
                unsafe {
                    match self.get_unchecked_mut() {
                        $enum_name::$head_variant(x) => ::core::pin::Pin::new_unchecked(x).poll_flush(cx),
                        $(
                            $enum_name::$tail_variants(x) => ::core::pin::Pin::new_unchecked(x).poll_flush(cx),
                        )*
                    }
                }
            }

            fn poll_close(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>) ->::core::task::Poll<Result<(), Self::Error>> {
                unsafe {
                    match self.get_unchecked_mut() {
                        $enum_name::$head_variant(x) => ::core::pin::Pin::new_unchecked(x).poll_close(cx),
                        $(
                            $enum_name::$tail_variants(x) => ::core::pin::Pin::new_unchecked(x).poll_close(cx),
                        )*
                    }
                }
            }
        }
    }
);

impl_one_of!(OneOf8; One, Two, Three, Four, Five, Six, Seven, Eight);
impl_one_of!(OneOf7; One, Two, Three, Four, Five, Six, Seven);
impl_one_of!(OneOf6; One, Two, Three, Four, Five, Six);
impl_one_of!(OneOf5; One, Two, Three, Four, Five);
impl_one_of!(OneOf4; One, Two, Three, Four);
impl_one_of!(OneOf3; One, Two, Three);
impl_one_of!(OneOf2; One, Two);

#[cfg(test)]
mod tests {
    use core::task::Poll;

    use futures::{pin_mut, poll};
    use futures::executor::block_on;

    use super::*;

    #[test]
    fn it_works() {
        let one_of_7 = match 1 {
            0 => OneOf7::One(async { 1 }),
            1 => OneOf7::Two(async { 2 }),
            2 => OneOf7::Three(async { 3 }),
            3 => OneOf7::Four(async { 4 }),
            4 => OneOf7::Five(async { 5 }),
            6 => OneOf7::Six(async { 6 }),
            _ => OneOf7::Seven(async { 7 }),
        };

        block_on(async {
            pin_mut!(one_of_7);
            assert_eq!(Poll::Ready(2), poll!(&mut one_of_7));
        });
    }

    #[test]
    fn test_readme_deps() {
        version_sync::assert_markdown_deps_updated!("README.md");
    }

    #[test]
    fn test_html_root_url() {
        version_sync::assert_html_root_url_updated!("src/lib.rs");
    }
}
