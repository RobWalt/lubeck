use crate::traits::Functor;

impl<'a, In, Out> Functor<'a, In, Out> for Option<In> {
    fn fmap<F>(self, f: F) -> Self::OutputType
    where
        F: FnOnce(In) -> Out,
    {
        self.map(f)
    }
}
