use crate::traits::Functor;

impl<A> Functor<A> for Option<A> {
    fn fmap<F, B>(self, f: F) -> Self::Type<B>
    where
        F: Fn(A) -> B,
    {
        self.map(f)
    }
}
