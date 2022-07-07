use crate::traits::Applicative;

use super::def::Reader;

impl<R: 'static, A: 'static> Applicative<A> for Reader<R, A> {
    fn app<F, B>(self, f: Self::Type<F>) -> Self::Type<B>
    where
        F: Fn(A) -> B + 'static,
    {
        Reader::<R, B>::new(move |env| f.run(env.clone())(self.run(env)))
    }
}
