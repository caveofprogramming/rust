#![allow(unused)]

fn main() {
    // String, str
    let v:Vec<i32> = Vec::new();
    let s = String::new();
    let text = String::from("Hello");

    let mut text = "Hello".to_string();
    text.push_str(" to you");
    text.push('!');
    println!("{text}");

    let s1 = "Happy ".to_string();
    let s2 = "New Year".to_string();

    let s3 = s1 + &s2;

    // println!("{s1}");
    println!("{s2}");
    println!("{s3}");

    let s4 = &s3[0..4];
    println!("{s4}");

    let non_english = "őrült".to_string();
    let slice = &non_english[0..2];
    println!("{slice}");

    for c in non_english.chars() {
        print!("{c} ");
    }
    println!();
    for c in non_english.bytes() {
        print!("{c} ");
    }
    println!();

    println!("{}", non_english.chars().nth(2).unwrap());

    let s1 = "Hello";
    let s2 = "to";
    let s3 = "you";

    let s4 = s1.to_owned() + &s2 + &s3;
    println!("{s4}");
    
    let s4 = format!("{s1} {s2} {s3}");
    println!("{s4}");
}
