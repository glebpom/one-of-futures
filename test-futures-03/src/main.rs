use one_of_futures::{impl_one_of, OneOf2, OneOf7};

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
    use one_of_futures::{AsyncRead, AsyncWrite};
    use std::io::Cursor;

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

    fn only_accepts_asyncread(_: impl AsyncRead) {}

    fn only_accepts_asyncwrite(_: impl AsyncWrite) {}

    #[test]
    fn it_work_03_io() {
        let read_cursor = Cursor::new(vec![0u8, 1, 2]);
        let write_cursor = Cursor::new(vec![9u8, 8, 7]);

        let one_of_2 = match 1 {
            1 => OneOf2::One(read_cursor),
            _ => OneOf2::Two(write_cursor),
        };

        only_accepts_asyncread(one_of_2.clone());
        only_accepts_asyncwrite(one_of_2);
    }
}
