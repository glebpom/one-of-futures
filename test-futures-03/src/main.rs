use one_of_futures::{impl_one_of, OneOf7};

impl_one_of!(MyEither;
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
    use core::task::Poll;
    use futures::executor::block_on;
    use futures::{pin_mut, poll};

    use super::*;

    #[test]
    fn it_works_03() {
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
}
