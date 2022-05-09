use lubeck::{Monad, Return};

#[test]
fn monad_option() {
    fn add_one(x: i32) -> Option<i32> {
        Some(x + 1)
    }

    let x = Option::r#return(1);
    let y = x.bind(add_one);

    assert_eq!(Some(2), y);
}
