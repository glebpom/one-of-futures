/// Macro to implement custom `OneOf` type.
///
/// Example:
/// ```rust
/// use one_of_futures::impl_one_of_01;
///
/// impl_one_of_01!(MyEither;
///   Left,
///   Right
/// );
///
/// fn main() {
/// #  use futures_01::IntoFuture;
///   let either = match 1 {
///       0 => MyEither::Left(Result::<_, ()>::Ok(1).into_future()),
///       _ => MyEither::Right(Result::<(), _>::Err("hello").into_future()),
///   };
/// }
/// ```
#[macro_export]
macro_rules! impl_one_of_01 (
    ($enum_name:ident ; $head_variant:ident, $($tail_variants:ident),*) => {
        /// Combines multiple different futures, streams, or sinks having the
        /// same associated types into a single type.
        #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum $enum_name<$head_variant, $( $tail_variants ),*> {
            $head_variant($head_variant),
            $( $tail_variants($tail_variants) ),*
        }

        impl<$head_variant, $($tail_variants),*> ::futures_01::future::Future for
            $enum_name<$head_variant, $($tail_variants ),*>
            where
                $head_variant: ::futures_01::future::Future,
                $( $tail_variants: ::futures_01::future::Future<Item=$head_variant::Item, Error=$head_variant::Error> ),* {

            type Item = $head_variant::Item;
            type Error = $head_variant::Error;

            fn poll(&mut self) -> ::futures_01::Poll<Self::Item, Self::Error> {
                match self {
                    $enum_name::$head_variant(ref mut x) => x.poll(),
                    $(
                        $enum_name::$tail_variants(ref mut x) => x.poll(),
                    )*
                }
            
            }
        }

         impl<$head_variant, $($tail_variants),*> ::futures_01::stream::Stream for
            $enum_name<$head_variant, $($tail_variants ),*>
            where
                $head_variant: ::futures_01::stream::Stream,
                $( $tail_variants: ::futures_01::stream::Stream<Item=$head_variant::Item, Error=$head_variant::Error> ),* {

            type Item = $head_variant::Item;
            type Error = $head_variant::Error;

            fn poll(&mut self) -> ::futures_01::Poll<Option<$head_variant::Item>, Self::Error> {
                match self {
                    $enum_name::$head_variant(ref mut x) => x.poll(),
                    $(
                        $enum_name::$tail_variants(ref mut x) => x.poll(),
                    )*
                }
            
            }
        }

        impl<$head_variant, $($tail_variants),*> ::futures_01::sink::Sink for
            $enum_name<$head_variant, $( $tail_variants ),*>
            where
                $head_variant: ::futures_01::sink::Sink,
                $( $tail_variants: ::futures_01::sink::Sink<SinkItem = $head_variant::SinkItem, SinkError=$head_variant::SinkError> ),* {
            
            type SinkItem = $head_variant::SinkItem;
            type SinkError = $head_variant::SinkError;

            fn start_send(&mut self, item: Self::SinkItem) -> ::futures_01::StartSend<Self::SinkItem, Self::SinkError> {
                match self {
                    $enum_name::$head_variant(ref mut x) => x.start_send(item),
                    $(
                        $enum_name::$tail_variants(ref mut x) => x.start_send(item),
                    )*
                }
            }
            
            fn poll_complete(&mut self) -> ::futures_01::Poll<(), Self::SinkError> {
                match self {
                    $enum_name::$head_variant(x) => x.poll_complete(),
                    $(
                        $enum_name::$tail_variants(x) => x.poll_complete(),
                    )*
                }
                
            }
        }
    }
);

impl_one_of_01!(OneOf8; One, Two, Three, Four, Five, Six, Seven, Eight);
impl_one_of_01!(OneOf7; One, Two, Three, Four, Five, Six, Seven);
impl_one_of_01!(OneOf6; One, Two, Three, Four, Five, Six);
impl_one_of_01!(OneOf5; One, Two, Three, Four, Five);
impl_one_of_01!(OneOf4; One, Two, Three, Four);
impl_one_of_01!(OneOf3; One, Two, Three);
impl_one_of_01!(OneOf2; One, Two);

#[cfg(test)]
mod tests {
    use futures_01::executor::spawn;
    use futures_01::IntoFuture;

    use super::*;

    #[test]
    fn it_works_01() {
        let one_of_7 = match 1 {
            0 => OneOf7::One(Result::<_, ()>::Ok(1).into_future()),
            1 => OneOf7::Two(Ok(2).into_future()),
            2 => OneOf7::Three(Ok(3).into_future()),
            3 => OneOf7::Four(Ok(4).into_future()),
            4 => OneOf7::Five(Ok(5).into_future()),
            6 => OneOf7::Six(Ok(6).into_future()),
            _ => OneOf7::Seven(Ok(7).into_future()),
        };

        let two = spawn(one_of_7).wait_future().unwrap();
        assert_eq!(two, 2);
    }
}
