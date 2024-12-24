fn main() {
    let value = Some(7);

    if let Some(number) = value {
        println!("Value is: {number:?}");
    };

    let something = if let Some(number) = value { number } else { 0 };

    println!("Something: {something:?}");
}
