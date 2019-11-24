use one_of_futures::futures_01::OneOf7;
use one_of_futures::impl_one_of_01;

impl_one_of_01!(MyEither;
  Left,
  Right
);

fn main() {
    let _either = match 1 {
        0 => MyEither::Left(async { 1 }),
        _ => MyEither::Right(async { 2 }),
    };
}

#[cfg(test)]
mod tests {
    use futures::executor::spawn;
    use futures::IntoFuture;

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
