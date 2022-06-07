use crate::common::State;
use crate::traits::{Applicative, Functor, Pure, HKT};

impl<T> Pure<T> for Option<T> {
    fn pure(x: T) -> Self::OutputType {
        Some(x)
    }
}

impl<'a, In, Out> Applicative<'a, In, Out> for Option<In> {
    fn apply(self, f: Self::FunctionContextType) -> <Self as HKT<In, Out>>::OutputType {
        match f {
            Some(func) => self.fmap(func),
            None => None,
        }
    }
}

impl<'a, S: 'a, A: 'a> Pure<A> for State<'a, S, A> {
    fn pure(action: A) -> Self::OutputType {
        Self::pure_value(action)
    }
}

impl<'a, S: 'a, AIn: 'a, AOut: 'a> Applicative<'a, AIn, AOut> for State<'a, S, AIn> {
    fn apply(self, f: Self::FunctionContextType) -> <Self as HKT<AIn, AOut>>::OutputType {
        State::<'a, S, AOut>::crate_intern_new(Box::new(|state| {
            let (action_func, state) = f.run(state);
            self.fmap(action_func).run(state)
        }))
    }
}
