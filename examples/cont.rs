use lubeck::traits::*;

use lubeck::common::cont::Cont;

fn square(x: i32) -> i32 {
    x * x
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn square_cont<R>(x: i32) -> Cont<R, i32> {
    Cont::pure(square(x))
}

fn add_cont<R>(x: i32, y: i32) -> Cont<R, i32> {
    Cont::pure(add(x, y))
}

fn pythagoras<R: 'static>(x_arg: i32, y_arg: i32) -> Cont<R, i32> {
    square_cont(x_arg).bind(move |x| square_cont(y_arg).bind(move |y| add_cont(x, y)))
}

fn main() {
    {
        // Functor example
        let id = Box::new(|x| x);
        let to_string = Box::new(|x: i32| x.to_string() + " λ");
        let sqr_cont = square_cont(5).fmap(to_string);
        let sqr_cont_res = sqr_cont.eval(id);
        println!("square res:{}", sqr_cont_res);
    }

    {
        // Monad example
        let id = Box::new(|x| x);
        let pythagoras_cont = pythagoras(3, 4);
        let pythagoras_cont_res = pythagoras_cont.eval(id);
        println!("pythagoras res:{:?}", pythagoras_cont_res);
    }
}
