use crate::traits::{GenType, Monad};

pub struct Cont<R, A> {
    run_cont: Box<dyn FnOnce(Box<dyn FnOnce(A) -> R>) -> R + 'static>,
}

impl<R, A: 'static> Cont<R, A> {
    pub fn new<F>(f: F) -> Self
    where
        F: FnOnce(Box<dyn FnOnce(A) -> R>) -> R + 'static,
    {
        Self {
            run_cont: Box::new(f),
        }
    }

    pub fn pure(x: A) -> Self {
        Cont {
            run_cont: Box::new(move |f| f(x)),
        }
    }

    pub fn eval(self, f: Box<dyn FnOnce(A) -> R>) -> R {
        (self.run_cont)(f)
    }
}

impl<R, A> GenType for Cont<R, A> {
    type Type<B> = Cont<R, B>;
}
