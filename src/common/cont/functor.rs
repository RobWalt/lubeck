use crate::traits::Functor;

use super::def::Cont;

impl<R: 'static, A: 'static> Functor<A> for Cont<R, A> {
    fn fmap<F, B: 'static>(self, f: F) -> Self::Type<B>
    where
        F: Fn(A) -> B + 'static,
    {
        Cont::<R, B>::new(move |k: Box<dyn FnOnce(B) -> R>| -> R {
            self.eval(Box::new(move |x: A| -> R { k(f(x)) }))
        })
    }
}
