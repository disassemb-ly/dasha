use std::mem;

pub trait ReadLe: Sized {
    fn read_le<'a, I>(i: I) -> Option<Self>
    where
        I: IterExt<'a>;
}

macro_rules! impl_read_le {
    ( $( $ty:ty ),+ ) => {
        $(
            impl ReadLe for $ty {
                fn read_le<'a, I>(i: I) -> Option<Self>
                where
                    I: IterExt<'a>,
                {
                    const LEN: usize = mem::size_of::<$ty>();
                    let vec = i.take(LEN).copied().collect::<Vec<_>>();
                    if vec.len() == LEN {
                        let data = {
                            let mut data = [0; LEN];
                            data.copy_from_slice(&vec[..]);
                            data
                        };
                        Some(<$ty>::from_le_bytes(data))
                    } else {
                        None
                    }
                }
            }
        )*
    }
}

impl_read_le!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);

pub trait IterExt<'a>: Iterator<Item = &'a u8> {
    fn read_le<T>(&mut self) -> Option<T>
    where
        T: ReadLe,
    {
        T::read_le(self)
    }
}

impl<'a, T> IterExt<'a> for T where T: Iterator<Item = &'a u8> {}

#[test]
fn test_iter_read_le() {
    assert_eq!([].iter().read_le::<i16>(), None);
    assert_eq!([0xff].iter().read_le::<i16>(), None);
    assert_eq!([0xff, 0xff].iter().read_le::<i16>(), Some(-1));
    assert_eq!([].iter().read_le::<u32>(), None);
    assert_eq!([0x00].iter().read_le::<u32>(), None);
    assert_eq!([0x00, 0x00].iter().read_le::<u32>(), None);
    assert_eq!([0x00, 0x00, 0x00].iter().read_le::<u32>(), None);
    assert_eq!([0x00, 0x00, 0x00, 0x00].iter().read_le::<u32>(), Some(0));
}
