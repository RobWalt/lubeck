use crate::traits::Monoid;

macro_rules! integer_monoid_impl {
    ($T:ty) => {
        impl Monoid for $T {
            fn mempty() -> Self {
                0
            }
        }
    };
}

integer_monoid_impl!(i8);
integer_monoid_impl!(i16);
integer_monoid_impl!(i32);
integer_monoid_impl!(i64);

integer_monoid_impl!(u8);
integer_monoid_impl!(u16);
integer_monoid_impl!(u32);
integer_monoid_impl!(u64);
