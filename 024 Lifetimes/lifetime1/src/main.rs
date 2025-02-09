#![allow(unused)]

fn get_text<'a>(s: &'a str) -> &'a str {
    s
}

fn main() {
    let text = "Hello";
    let result = get_text(text);
    println!("{result}");
}
