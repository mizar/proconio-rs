use crate::source::{ReadSource, Source};
use std::io::BufRead;

impl ReadSource for String {
    type Output = String;
    fn read<R: BufRead>(source: &mut Source<R>) -> String {
        source.next_token().collect()
    }
}

pub type Chars = Vec<char>;
impl ReadSource for Chars {
    type Output = Chars;
    fn read<R: BufRead>(source: &mut Source<R>) -> Chars {
        source.next_token().collect()
    }
}

pub type Bytes = Vec<u8>;
impl ReadSource for Bytes {
    type Output = Bytes;
    fn read<R: BufRead>(source: &mut Source<R>) -> Bytes {
        source.next_token().map(|x| x as _).collect()
    }
}

macro_rules! impl_read_source_for_primitives {
    ($($ty:ty)*) => {
        $(
            impl ReadSource for $ty  {
                type Output = $ty;
                fn read<R: BufRead>(source: &mut Source<R>) -> $ty {
                    let s = <String as ReadSource>::read(source);
                    s.parse().expect("failed to parse")
                }
            }
        )*
    }
}

impl_read_source_for_primitives! {
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    char bool f32 f64
}
