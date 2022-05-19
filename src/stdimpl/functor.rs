use crate::traits::Functor;

impl<In, Out> Functor<In, Out> for Option<In> {
    fn fmap(self, f: Self::FunctionType) -> Self::OutputType {
        self.map(f)
    }
}
