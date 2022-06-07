use crate::common::State;
use crate::traits::Functor;

impl<'a, In, Out> Functor<'a, In, Out> for Option<In> {
    fn fmap<F>(self, f: F) -> Self::OutputType
    where
        F: Fn(In) -> Out,
    {
        self.map(f)
    }
}

impl<'a, StateType: 'a, InActionType: 'a, OutActionType: 'a>
    Functor<'a, InActionType, OutActionType> for State<'a, StateType, InActionType>
{
    fn fmap<F>(self, f: F) -> Self::OutputType
    where
        F: Fn(InActionType) -> OutActionType + 'a,
    {
        let inner = self.run_state;
        Self::OutputType::crate_intern_new(Box::new(move |state: StateType| {
            let (inner_action, inner_state) = inner(state);
            let action = f(inner_action);
            (action, inner_state)
        }))
    }
}
