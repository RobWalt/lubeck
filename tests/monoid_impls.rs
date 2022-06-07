use lubeck::traits::Monoid;
use proptest::prelude::*;

macro_rules! monoid_check {
    ($T:ty) => {
        ::paste::paste! {
            monoid_check!($T, [<mempty_is_left_unit_$T>], [<mempty_is_right_unit_$T>]);
        }
    };
    ($T:ty, $left:ident, $right:ident) => {
        proptest! {
            #![proptest_config(ProptestConfig {
              cases: 10000, .. ProptestConfig::default()
            })]

            #[test]
            fn $left(a: $T) {
                prop_assert_eq!(<$T>::mempty().mappend(a), a);
            }

            #[test]
            fn $right(a: $T) {
                prop_assert_eq!(a.mappend(<$T>::mempty()), a);
            }
        }
    };
}

monoid_check!(i8);
monoid_check!(i16);
monoid_check!(i32);
monoid_check!(i64);

monoid_check!(u8);
monoid_check!(u16);
monoid_check!(u32);
monoid_check!(u64);
