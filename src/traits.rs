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

pub trait GenType {
    type Type<T>;
}

pub trait Functor<A>: GenType {
    fn fmap<F, B>(self, f: F) -> Self::Type<B>
    where
        F: Fn(A) -> B + 'static;
}

pub trait Pure: GenType {
    fn pure<T: 'static>(t: T) -> Self::Type<T>;
}

pub trait Applicative<A>: Functor<A> + GenType {
    fn app<F, B>(self, f: Self::Type<F>) -> Self::Type<B>
    where
        F: Fn(A) -> B + 'static;
}

pub trait Monad<A>: Functor<A> + GenType {
    fn bind<F, B>(self, f: F) -> Self::Type<B>
    where
        F: Fn(A) -> Self::Type<B> + 'static;
}
