#![allow(unused)]

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let text1 = "Hi";
    let result;

    {
        let text2:&'static str = "Hello"; //String::from("Hi").as_str();
        result = longest(text1, text2);
    }

    println!("{result}");
}
