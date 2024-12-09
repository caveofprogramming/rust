/*
 * 1. Every value has an owner
 * 2. There can only be one owner at a time.
 * 3. When the owners goes out of scope, the value will be dropped
 * 
 * If you have a mutable reference to a value, you can't have any other
 * valid references to it at that point.
 */

fn main() {
    let mut text = String::from("Hello");
    text = transfer(text);
    println!("Transfer: {text}");

    borrow(&text);
    println!("Borrow: {text}");

    mutable_borrow(&mut text);
    println!("Mutable borrow: {text}");

    let mut greeting = String::from("Hi");

    let r1: &String = &greeting;
    let r2: &String = &greeting;
    let r3: &String = &greeting;

    println!("{r1}, {r2}, {r3}");

    /* Can't do this.
    let m1: &mut String = &mut greeting;
    let m2: &mut String = &mut greeting;
    println!("{m1}, {m2}");
    */

    /* Can't do this.
    let m1: &mut String = &mut greeting;
    println!("{r1}, {m1}");
    */

    let m1: &mut String = &mut greeting;
    println!("{m1}");
}

fn mutable_borrow(text: &mut String) {
    text.push_str(" there!");
    println!("Mutable borrow in function: {text}");
}

fn borrow(text: &String) {
    println!("Borrow in function: {text}");
}

fn transfer(text: String) -> String {
    text
}
