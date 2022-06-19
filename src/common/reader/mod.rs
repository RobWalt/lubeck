use crate::traits::Functor;

mod applicative;
mod functor;
mod higherkinded;
mod monad;

pub struct Reader<'a, R, A> {
    run_reader: Box<dyn Fn(R) -> A + 'a>,
}

impl<'a, R: 'a, A: 'a> Reader<'a, R, A> {
    pub fn reader<F>(f: F) -> Self
    where
        F: Fn(R) -> A + 'a,
    {
        Self {
            run_reader: Box::new(f),
        }
    }

    pub fn run(&self, env: R) -> A {
        (self.run_reader)(env)
    }

    #[must_use]
    pub fn ask() -> Reader<'a, R, R> {
        Reader::<'a, R, R> {
            run_reader: Box::new(|x| x),
        }
    }

    pub fn asks<F>(f: F) -> Self
    where
        F: Fn(R) -> A + 'a,
    {
        Self::ask().fmap(f)
    }

    pub fn with_reader<F, S: 'a>(f: F, pre_reader: Reader<'a, S, A>) -> Self
    where
        F: Fn(R) -> S + 'a,
    {
        Reader::reader(move |env_r| pre_reader.run(f(env_r)))
    }

    pub fn local<F>(f: F, adjust_reader: Self) -> Self
    where
        F: Fn(R) -> R + 'a,
    {
        Reader::reader(move |env| adjust_reader.run(f(env)))
    }
}

