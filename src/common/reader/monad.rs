use crate::traits::Monad;

use super::Reader;

impl<R: 'static, A: 'static> Monad<A> for Reader<R, A> {
    fn bind<F, B>(self, f: F) -> Self::Type<B>
    where
        F: Fn(A) -> Self::Type<B> + 'static,
    {
        Reader::<R, B>::new(move |env| f(self.run(env.clone())).run(env))
    }
}
