#![allow(unused)]

use std::ops::Deref;

fn main() {
    struct Thing<T>(T);
    let thing:Thing<i32> = Thing::<i32>(7);
    println!("{}", thing.0);

    impl<T> Thing<T> {
        fn new(value:T)->Thing<T> {
            Thing::<T>(value)
        }
    }

    impl<T> Deref for Thing<T> {
        type Target = T;

        fn deref(&self)->&Self::Target {
            &self.0
        }
    }

    let thing = Thing::new(99);
    println!("{}", *thing);

    print_value(&thing);
}

fn print_value(value:&i32) {
    println!("{}", value);
}
