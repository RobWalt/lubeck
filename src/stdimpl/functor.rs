use crate::common::State;
use crate::traits::Functor;

impl<In, Out> Functor<In, Out> for Option<In> {
    fn fmap(self, f: Self::FunctionType) -> Self::OutputType {
        self.map(f)
    }
}

impl<StateType: 'static, InActionType: 'static, OutActionType: 'static>
    Functor<InActionType, OutActionType> for State<StateType, InActionType>
{
    fn fmap(self, f: Self::FunctionType) -> Self::OutputType {
        let inner = self.run_state;
        Self::OutputType::new(Box::new(move |state: StateType| {
            let (inner_action, inner_state) = inner(state);
            let action = f(inner_action);
            (action, inner_state)
        }))
    }
}
