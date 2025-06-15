#![allow(unused)]

use std::rc::Rc;
use std::rc::Weak;

fn main() {
    let strong_ref1  = Rc::new(7);

    println!(
        "weak: {} strong: {}",
        Rc::weak_count(&strong_ref1),
        Rc::strong_count(&strong_ref1),
    );

    let weak_ref1:Weak<i32> = Rc::downgrade(&strong_ref1);

    println!(
        "weak: {} strong: {}",
        Rc::weak_count(&strong_ref1),
        Rc::strong_count(&strong_ref1),
    );

    if let Some(r) = weak_ref1.upgrade() {
        println!("value: {}", r);

        println!(
            "weak: {} strong: {}",
            Rc::weak_count(&strong_ref1),
            Rc::strong_count(&strong_ref1),
        );
    }

    println!(
        "weak: {} strong: {}",
        Rc::weak_count(&strong_ref1),
        Rc::strong_count(&strong_ref1),
    );
}
