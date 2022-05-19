use crate::traits::{HKTLight, HKT};

impl<In, Out> HKT<In, Out> for Option<In> {
    type InputType = Self;
    type OutputType = Option<Out>;
    type FunctionContextType = Option<fn(In) -> Out>;
    type LiftingFunctionType = fn(In) -> Option<Out>;

    // TODO remove and use unstable default associated types
    type FunctionType = fn(In) -> Out;
}

impl<T> HKTLight<T> for Option<T> {
    type OutputType = Self;
}
