pub trait SemiGroup {
    #[must_use]
    fn mappend(self, other: Self) -> Self;
}

pub trait Monoid: SemiGroup {
    fn mempty() -> Self;
    #[must_use]
    fn mappend(self, other: Self) -> Self
    where
        Self: Sized,
    {
        SemiGroup::mappend(self, other)
    }
}

pub trait HKT<In, Out> {
    type InputType;
    type OutputType;
    type FunctionContextType;
    type LiftingFunctionType;
}

pub trait HKTLight<T> {
    type OutputType;
}

pub trait Functor<'a, In, Out>: HKT<In, Out> {
    fn fmap<F>(self, f: F) -> Self::OutputType
    where
        F: Fn(In) -> Out + 'a;
}

pub trait Pure<T>: HKTLight<T> {
    // TODO: Think about pure and if it is general enough
    fn pure(x: T) -> Self::OutputType;
}

pub trait Applicative<'a, In, Out>: Functor<'a, In, Out> + Pure<In> {
    fn apply(self, f: Self::FunctionContextType) -> <Self as HKT<In, Out>>::OutputType;
}

pub trait Return<T>: Pure<T> {
    fn r#return(x: T) -> <Self as HKTLight<T>>::OutputType {
        <Self as Pure<T>>::pure(x)
    }
}

pub trait Monad<'a, In, Out>: Applicative<'a, In, Out> + Return<In> {
    fn bind(self, f: Self::LiftingFunctionType) -> <Self as HKT<In, Out>>::OutputType;
}
