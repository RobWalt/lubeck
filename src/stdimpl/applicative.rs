use crate::traits::{Applicative, Functor, Pure, HKT};

impl<T> Pure<T> for Option<T> {
    fn pure(x: T) -> Self::OutputType {
        Some(x)
    }
}

impl<'a, In, Out> Applicative<'a, In, Out> for Option<In> {
    fn apply(self, f: Self::FunctionContextType) -> <Self as HKT<In, Out>>::OutputType {
        match f {
            Some(func) => self.fmap(func),
            None => None,
        }
    }
}
