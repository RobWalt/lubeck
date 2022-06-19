use crate::traits::{HKTLight, HKT};

use super::Reader;

impl<'a, R: 'a, In: 'a, Out: 'a> HKT<In, Out> for Reader<'a, R, In> {
    type InputType = Self;
    type OutputType = Reader<'a, R, Out>;
    type FunctionContextType = Reader<'a, R, fn(In) -> Out>;
    type LiftingFunctionType = fn(In) -> Reader<'a, R, Out>;
}

impl<'a, R, A> HKTLight<A> for Reader<'a, R, A> {
    type OutputType = Self;
}
