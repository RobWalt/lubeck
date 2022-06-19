use crate::traits::{Monad, Return, HKT};

impl<T> Return<T> for Option<T> {}

impl<'a, In, Out> Monad<'a, In, Out> for Option<In> {
    fn bind(self, f: Self::LiftingFunctionType) -> <Self as HKT<In, Out>>::OutputType {
        self.and_then(f)
    }
}
