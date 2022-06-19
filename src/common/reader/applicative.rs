use crate::traits::{Applicative, Pure};

use super::Reader;

impl<'a, R: 'a, A: Clone + 'a> Pure<A> for Reader<'a, R, A> {
    fn pure(x: A) -> Self::OutputType {
        Self::OutputType::reader(move |_| x.clone())
    }
}

impl<'a, R: 'a + Clone, In: 'a + std::clone::Clone, Out: 'a> Applicative<'a, In, Out>
    for Reader<'a, R, In>
{
    fn apply(
        self,
        f: Self::FunctionContextType,
    ) -> <Self as crate::traits::HKT<In, Out>>::OutputType {
        Reader::reader(move |env: R| f.run(env.clone())(self.run(env)))
    }
}
