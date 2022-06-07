use lubeck::traits::SemiGroup;
use proptest::prelude::*;

macro_rules! semigroup_check {
    ($T:ty) => {
        ::paste::paste! {
            semigroup_check!($T, [<mappend_is_associative_$T>]);
        }
    };
    ($T:ty, $id:ident) => {
        proptest! {
            #![proptest_config(ProptestConfig {
              cases: 10000, .. ProptestConfig::default()
            })]

            #[test]
            fn $id(a: $T, b: $T, c: $T) {
                prop_assert_eq!(a.mappend(b).mappend(c), a.mappend(b.mappend(c)))
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
