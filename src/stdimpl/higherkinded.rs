use crate::common::State;
use crate::traits::{HKTLight, HKT};

impl<In, Out> HKT<In, Out> for Option<In> {
    type InputType = Self;
    type OutputType = Option<Out>;
    type FunctionContextType = Option<fn(In) -> Out>;
    type LiftingFunctionType = fn(In) -> Option<Out>;
}

impl<T> HKTLight<T> for Option<T> {
    type OutputType = Self;
}

impl<'a, StateType, InActionType, OutActionType> HKT<InActionType, OutActionType>
    for State<'a, StateType, InActionType>
{
    type InputType = Self;
    type OutputType = State<'a, StateType, OutActionType>;
    type FunctionContextType = State<'a, StateType, Box<dyn FnOnce(InActionType) -> OutActionType>>;
    type LiftingFunctionType = fn(InActionType) -> State<'a, StateType, OutActionType>;
}

impl<'a, S, T> HKTLight<T> for State<'a, S, T> {
    type OutputType = Self;
}
