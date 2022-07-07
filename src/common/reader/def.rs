use std::rc::Rc;

use crate::traits::GenType;

pub struct Reader<R, A> {
    run_reader: Box<dyn Fn(Rc<R>) -> A + 'static>,
}

impl<R, A> Reader<R, A> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(Rc<R>) -> A + 'static,
    {
        Self {
            run_reader: Box::new(f),
        }
    }

    pub fn run(&self, env: Rc<R>) -> A {
        (self.run_reader)(env)
    }
}

impl<R, A> GenType for Reader<R, A> {
    type Type<B> = Reader<R, B>;
}
