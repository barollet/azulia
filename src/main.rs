mod rules;

use rules::*;

fn main() {
    let a: DrawBag = DrawBag::init_sized_bag(3);
    for t in a.into_iter() {
        println!("{}", t);
    }
    println!("Hello, world!");
}
