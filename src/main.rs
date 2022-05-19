use lubeck::traits::SemiGroup;

fn main() {
    let (a, b, c) = (5, 1, 00);
    let x = a.mappend(b).mappend(c) == a.mappend(b.mappend(c));
    println!("{x}");
}
