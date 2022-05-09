pub trait SemiGroup {
    fn mappend(self, other: Self) -> Self;
}

pub trait Monoid: SemiGroup {
    fn mempty() -> Self;
}

pub trait HKT<In, Out> {
    type InputType;
    type OutputType;
    type FunctionContextType;
    type LiftingFunctionType;

    type FunctionType;
    //TODO: use unstable default associated types
    //type FunctionType = fn(In) -> Out;
}

pub trait HKTLight<T> {
    type OutputType;
}

pub trait Functor<In, Out>: HKT<In, Out> {
    fn fmap(self, f: Self::FunctionType) -> Self::OutputType;
}

pub trait Pure<T>: HKTLight<T> {
    // TODO: Think about pure and if it is general enough
    fn pure(x: T) -> Self::OutputType;
}

pub trait Applicative<In, Out>: Functor<In, Out> + Pure<In> {
    fn apply(self, f: Self::FunctionContextType) -> <Self as HKT<In, Out>>::OutputType;
}

pub trait Return<T>: Pure<T> {
    fn r#return(x: T) -> <Self as HKTLight<T>>::OutputType {
        <Self as Pure<T>>::pure(x)
    }
}

pub trait Monad<In, Out>: Applicative<In, Out> + Return<In> {
    fn bind(self, f: Self::LiftingFunctionType) -> <Self as HKT<In, Out>>::OutputType;
}

// ==== Impls for Option ====

impl SemiGroup for i32 {
    fn mappend(self, other: Self) -> Self {
        self + other
    }
}

impl Monoid for i32 {
    fn mempty() -> Self {
        0
    }
}

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

impl<In, Out> Functor<In, Out> for Option<In> {
    fn fmap(self, f: Self::FunctionType) -> Self::OutputType {
        self.map(|x| f(x))
    }
}

impl<T> Pure<T> for Option<T> {
    fn pure(x: T) -> Self::OutputType {
        Some(x)
    }
}

impl<In, Out> Applicative<In, Out> for Option<In> {
    fn apply(self, f: Self::FunctionContextType) -> <Self as HKT<In, Out>>::OutputType {
        match f {
            Some(func) => self.fmap(func),
            None => None,
        }
    }
}

impl<T> Return<T> for Option<T> {}

impl<In, Out> Monad<In, Out> for Option<In> {
    fn bind(self, f: Self::LiftingFunctionType) -> <Self as HKT<In, Out>>::OutputType {
        self.map(|x| f(x)).flatten()
    }
}
