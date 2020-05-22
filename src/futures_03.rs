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

        impl<$head_variant, $($tail_variants),*> $crate::FusedFuture for
            $enum_name<$head_variant, $($tail_variants ),*>
            where
                $head_variant: $crate::FusedFuture,
                $( $tail_variants: $crate::FusedFuture<Output=$head_variant::Output> ),* {

            fn is_terminated(&self) -> bool {
                match self {
                    $enum_name::$head_variant(x) => x.is_terminated(),
                    $(
                        $enum_name::$tail_variants(x) => x.is_terminated(),
                    )*
                }
            }
        }

         impl<$head_variant, $($tail_variants),*> $crate::Stream for
            $enum_name<$head_variant, $($tail_variants ),*>
            where
                $head_variant: $crate::Stream,
                $( $tail_variants: $crate::Stream<Item=$head_variant::Item> ),* {

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

        impl<$head_variant, $($tail_variants),*> $crate::FusedStream for
            $enum_name<$head_variant, $( $tail_variants ),*>
            where
                $head_variant: $crate::FusedStream,
                $( $tail_variants: $crate::FusedStream<Item=$head_variant::Item> ),* {

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
        impl<Item, $head_variant, $($tail_variants),*> $crate::Sink<Item> for
            $enum_name<$head_variant, $( $tail_variants ),*>
            where
                $head_variant: $crate::Sink<Item>,
                $( $tail_variants: $crate::Sink<Item, Error=$head_variant::Error> ),* {
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

        #[cfg(feature = "tokio")]
        impl<$head_variant, $($tail_variants), *> $crate::AsyncRead for
            $enum_name<$head_variant, $( $tail_variants),*>
            where
                $head_variant: $crate::AsyncRead,
                $( $tail_variants: $crate::AsyncRead ),* {

            fn poll_read(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>, buf: &mut [u8]) -> ::core::task::Poll<tokio::io::Result<usize>> {
                unsafe {
                    match self.get_unchecked_mut() {
                        $enum_name::$head_variant(x) => ::core::pin::Pin::new_unchecked(x).poll_read(cx, buf),
                        $(
                            $enum_name::$tail_variants(x) => ::core::pin::Pin::new_unchecked(x).poll_read(cx, buf),
                        )*
                    }
                }
            }

            unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [::core::mem::MaybeUninit<u8>]) -> bool {
                match self {
                    $enum_name::$head_variant(x) => x.prepare_uninitialized_buffer(buf),
                    $(
                        $enum_name::$tail_variants(x) => x.prepare_uninitialized_buffer(buf),
                    )*
                }
            }
        }

        #[cfg(feature = "tokio")]
        impl<$head_variant, $($tail_variants), *> $crate::AsyncWrite for
            $enum_name<$head_variant, $( $tail_variants),*>
            where
                $head_variant: $crate::AsyncWrite,
                $( $tail_variants: $crate::AsyncWrite ),* {
            fn poll_write(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>, buf: &[u8]) -> ::core::task::Poll<tokio::io::Result<usize>> {
                unsafe {
                    match self.get_unchecked_mut() {
                        $enum_name::$head_variant(x) => ::core::pin::Pin::new_unchecked(x).poll_write(cx, buf),
                        $(
                            $enum_name::$tail_variants(x) => ::core::pin::Pin::new_unchecked(x).poll_write(cx, buf),
                        )*
                    }
                }
            }

            fn poll_flush(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>) -> ::core::task::Poll<tokio::io::Result<()>> {
                unsafe {
                    match self.get_unchecked_mut() {
                        $enum_name::$head_variant(x) => ::core::pin::Pin::new_unchecked(x).poll_flush(cx),
                        $(
                            $enum_name::$tail_variants(x) => ::core::pin::Pin::new_unchecked(x).poll_flush(cx),
                        )*
                    }
                }
            }

            fn poll_shutdown(self: ::core::pin::Pin<&mut Self>, cx: &mut ::core::task::Context<'_>) -> ::core::task::Poll<tokio::io::Result<()>> {
                unsafe {
                    match self.get_unchecked_mut() {
                        $enum_name::$head_variant(x) => ::core::pin::Pin::new_unchecked(x).poll_shutdown(cx),
                        $(
                            $enum_name::$tail_variants(x) => ::core::pin::Pin::new_unchecked(x).poll_shutdown(cx),
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
