use crate::traits::Monad;

use super::def::Cont;

impl<R: 'static, A: 'static> Monad<A> for Cont<R, A> {
    fn bind<F, B: 'static>(self, f: F) -> Self::Type<B>
    where
        F: Fn(A) -> Self::Type<B> + 'static,
    {
        Cont::<R, B>::new(move |k: Box<dyn FnOnce(B) -> R>| -> R {
            self.eval(Box::new(move |x: A| -> R {
                (move |x: A| -> Cont<R, B> { f(x) }(x)).eval(k)
            }))
        })
    }
}
