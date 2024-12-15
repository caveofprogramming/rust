fn main() {
    let values = [0, 1, 2, 3, 4, 5];
    let subset: &[i32] = &values[1..4];

    println!("{:?}", subset);

    let value = subset[1];
    println!("{value}");

    let text = "It was the worst of times.";
    let mut text: String = String::from("It was the best of times.");

    let substring: &str = &text[11..15];
    text.clear(); // Causes a compilation error.
    println!("'{substring}'");
}
