#![allow(unused)]

use std::rc::Rc;

fn main() {
    struct Person {
        name: String,
        age: u8,
    }

    impl Drop for Person {
        fn drop(&mut self) {
            println!("{} dropped", self.name);
        }
    }

    let p1 = Rc::new(Person {
        name: String::from("Bob"),
        age: 32,
    });

    println!("ref count: {}", Rc::strong_count(&p1));
    
    let p2 = Rc::clone(&p1);
    println!("ref count: {}", Rc::strong_count(&p1));

    let p3 = Rc::clone(&p1);
    println!("ref count: {}", Rc::strong_count(&p1));
    
    drop(p1);
    println!("ref count: {}", Rc::strong_count(&p2));

    drop(p2);
    println!("ref count: {}", Rc::strong_count(&p3));

    drop(p3);

    println!("Completed.");
}
