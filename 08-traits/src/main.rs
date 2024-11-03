mod basket;
mod container;
mod stack;

use basket::Basket;
use stack::Stack;

fn main() {
    let b1 = Basket::<String>::new(String::from("Hi there!"));

    let b2 = Basket::new(10);

    let b3 = Basket::new(true);

    let s1 = Stack::new(vec![String::from("Hi!")]);

    let s2 = Stack::new(vec![1, 2, 3]);
}
