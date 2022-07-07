use crate::traits::Monad;

impl<A> Monad<A> for Option<A> {
    fn bind<F, B>(self, f: F) -> Self::Type<B>
    where
        F: Fn(A) -> Self::Type<B>,
    {
        self.and_then(f)
    }
}
