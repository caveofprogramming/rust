#![allow(unused)]

fn main() {
    let mut animals: Vec<&str> = Vec::new();

    animals.push("dog");
    animals.push("cat");

    println!("{animals:?}");

    let fruits = vec!["apple", "banana", "cherry"];
    println!("{fruits:?}");

    let animal: &str = &animals[1];
    println!("{animal}");

    let animal = animals.get(0);

    let dog = if let Some(value) = animal {
        value
    } else {
        "unknown"
    };

    println!("{dog}");
    println!();

    /**********************************/

    // We can't have a mutable reference to a vector
    // and an immutable reference at the same time.

    let cat = &animals[1];
    animals.push("mouse");

    // If we uncomment this, the code won't work.
    //println!("Cat: {cat}");

    /**********************************/

    // Looping over items ...

    for a in &animals {
        println!("Animal: {a}");
    }

    // We can modify items even while looping ...
    let mut numbers = vec![1, 2, 3];

    for i in &mut numbers {
        *i *= 3;
    }
    println!("{numbers:?}");

    /**********************************/

    // We can use enums to store different types in vectors.

    #[derive(Debug)]
    enum Person {
        Name(String),
        Id(i32),
    }

    let people = vec![Person::Name(String::from("Bob")), Person::Id(73)];
    println!("{people:?}");
}
