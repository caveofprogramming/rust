#![allow(unused)]
fn main() {
    let value = Box::new(5);
    println!("{value}");

    let values = Box::new([7; 100]);
    println!("{}", values[5]);

    enum Fruits {
        Apple(i32, i32),
        Banana,
    }

    let fruit = Fruits::Apple(7, 8);

    let values = match fruit {
        Fruits::Apple(x, y) => (x, y),
        _ => (0, 0),
    };

    println!("{} {}", values.0, values.1);

    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let values = List::Cons(3, Box::new(List::Cons(4, Box::new(List::Nil))));

    println!("{}", std::mem::size_of::<List>());
}
