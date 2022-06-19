use crate::traits::SemiGroup;

macro_rules! integer_semigroup_impl {
    ($T:ty) => {
        impl SemiGroup for $T {
            fn mappend(self, other: Self) -> Self {
                self.wrapping_add(other)
            }
        }
    };
}

integer_semigroup_impl!(i8);
integer_semigroup_impl!(i16);
integer_semigroup_impl!(i32);
integer_semigroup_impl!(i64);

integer_semigroup_impl!(u8);
integer_semigroup_impl!(u16);
integer_semigroup_impl!(u32);
integer_semigroup_impl!(u64);
