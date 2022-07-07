mod applicative;
mod functor;
mod monad;

pub use applicative::*;
pub use functor::*;
pub use monad::*;

use crate::traits::GenType;

impl<T> GenType for Option<T> {
    type Type<U> = Option<U>;
}
