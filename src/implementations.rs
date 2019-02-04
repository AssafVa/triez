use super::Decomposable;

impl Decomposable<char, std::vec::IntoIter<char>> for String {
    fn decompose(self) -> std::vec::IntoIter<char> {
        self.chars().collect::<Vec<_>>().into_iter()
    }
}

macro_rules! impl_decomposable_for_integer {
    ( $t:ty ) => {
        impl Decomposable<u8, std::vec::IntoIter<u8>> for $t {
            fn decompose(self) -> std::vec::IntoIter<u8> {
                let bytes : Box<[u8]> = Box::new(self.to_be_bytes());
                bytes.into_vec().into_iter()
            }
        }
    };
}

macro_rules! impl_decomposable_for_float {
    ( $t:ty ) => {
        impl Decomposable<u8, std::vec::IntoIter<u8>> for $t {
            fn decompose(self) -> std::vec::IntoIter<u8> {
                let bytes : Box<[u8]> = Box::new(self.to_bits().to_be_bytes());
                bytes.into_vec().into_iter()
            }
        }
    };
}

impl_decomposable_for_integer!(u16);
impl_decomposable_for_integer!(u32);
impl_decomposable_for_integer!(u64);
impl_decomposable_for_integer!(u128);

impl_decomposable_for_integer!(i16);
impl_decomposable_for_integer!(i32);
impl_decomposable_for_integer!(i64);
impl_decomposable_for_integer!(i128);

impl_decomposable_for_integer!(usize);
impl_decomposable_for_integer!(isize);

impl_decomposable_for_float!(f32);
impl_decomposable_for_float!(f64);
