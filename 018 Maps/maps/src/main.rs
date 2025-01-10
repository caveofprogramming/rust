use std::collections::HashMap;

fn main() {
    let mut people = HashMap::new();

    people.insert("Bob", 24);
    people.insert("Sue", 36);
    people.insert("Rita", 72);

    println!("{people:?}");

    for (name, age) in &people {
        println!("{name}: {age}");
    }

    let bob_age = people.get("Bob");

    println!("{}", bob_age.copied().unwrap_or(0));

    if let Some(value) = people.get("Sue") {
        println!("Sue is: {value}");
    };

    let key = "Some key".to_string();
    let value = "Some value".to_string();

    let mut test_map = HashMap::new();
    test_map.insert(key, value);

    //println!("{key}");
    //println!("{value}");

    people.insert("Bob", 48);
    println!("{people:?}");

    people.entry("Bob").or_insert(86);
    people.entry("Ethel").or_insert(22);
    println!("{people:?}");

    people.entry("Bob").and_modify(|age| *age += 100);
    println!("{people:?}");

    let age_ethel = people.entry("Ethel").or_insert(0);
    *age_ethel += 20;
    println!("{people:?}");
}
