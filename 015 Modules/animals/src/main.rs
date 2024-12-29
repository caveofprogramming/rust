#![allow(unused)]

mod animals {
    pub struct Animal {
        pub name: String,
        pub id: i32,
    }

    pub use menagerie::speak as multiple;

    pub fn speak() {
        println!("I'm an animal.");
    }

    pub mod cat {
        pub fn speak() {
            println!("I'm a cat.");
        }
    }

    pub mod dog {
        pub fn speak() {
            println!("I'm a dog.");
        }
    }

    pub mod menagerie {
        pub fn speak() {
            super::cat::speak();
            super::dog::speak();
        }
    }
}

fn main() {
    use crate::animals;

    animals::speak();

    let animal: animals::Animal = animals::Animal {
        name: String::from("Bob"),
        id: 7,
    };

    //use animals::speak;
    use animals::*;
    speak();

    use animals::cat;
    cat::speak();

    println!();
    use animals::menagerie;
    menagerie::speak();
    
    println!();
    animals::multiple();
}
