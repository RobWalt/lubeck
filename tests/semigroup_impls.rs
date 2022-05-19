use lubeck::traits::SemiGroup;

macro_rules! semigroup_check {
    ($T:ty) => {
        ::paste::paste! {
            semigroup_check!($T, [<mappend_is_associative_$T>]);
        }
    };
    ($T:ty, $id:ident) => {
        quickcheck::quickcheck! {
        fn $id(a: $T, b: $T, c: $T) -> bool {
                a.mappend(b).mappend(c)
                    ==
                a.mappend(b.mappend(c))
            }
        }
    };
}

semigroup_check!(i8);
semigroup_check!(i16);
semigroup_check!(i32);
semigroup_check!(i64);

semigroup_check!(u8);
semigroup_check!(u16);
semigroup_check!(u32);
semigroup_check!(u64);
