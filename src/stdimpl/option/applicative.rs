use crate::traits::{Applicative, Pure};

impl<A> Pure for Option<A> {
    fn pure<T>(t: T) -> Self::Type<T> {
        Some(t)
    }
}

impl<A> Applicative<A> for Option<A> {
    fn app<F, B>(self, f: Self::Type<F>) -> Self::Type<B>
    where
        F: Fn(A) -> B,
    {
        f.and_then(|f| self.map(f))
    }
}
