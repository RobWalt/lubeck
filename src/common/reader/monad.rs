use crate::traits::{Monad, Return};

use super::Reader;

impl<'a, R: 'a, A: Clone + 'a> Return<A> for Reader<'a, R, A> {}

impl<'a, R: 'a + Clone, In: Clone + 'a, Out: 'a> Monad<'a, In, Out> for Reader<'a, R, In> {
    fn bind(
        self,
        f: Self::LiftingFunctionType,
    ) -> <Self as crate::traits::HKT<In, Out>>::OutputType {
        Reader::reader(move |env: R| f(self.run(env.clone())).run(env))
    }
}
