use crate::traits::Functor;

use super::def::Reader;

impl<R: 'static, A: 'static> Functor<A> for Reader<R, A> {
    fn fmap<F, B>(self, f: F) -> Self::Type<B>
    where
        F: Fn(A) -> B + 'static,
    {
        Reader::<R, B>::new(move |env| f(self.run(env)))
    }
}
