use crate::traits::Functor;

use super::Reader;

impl<'a, R: 'a, In: 'a, Out: 'a> Functor<'a, In, Out> for Reader<'a, R, In> {
    fn fmap<F>(self, f: F) -> Self::OutputType
    where
        F: Fn(In) -> Out + 'a,
    {
        Self::OutputType {
            run_reader: Box::new(move |env| f(self.run(env))),
        }
    }
}
