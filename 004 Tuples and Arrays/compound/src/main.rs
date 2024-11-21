fn main() {
    let mut x = 5;
    // x = 2.3; Can't do this
    let x = 1.2;

    let things = (7, 1.2, "Hello");
    let value = things.0;
    let text = things.2;
    println!("Value: {value}");
    println!("Text: {text}");

    let values:(f32, i8, f32) = (1.23, 8, 5.2);
    let value = values.2;
    println!("Value: {value}");

    let (a, b, c) = values;
    println!("{a}, {b}, {c}");

    let y = ();

    let fruits = ["apple", "orange", "banana"];
    let fruit = fruits[1];
    println!("Fruit: {fruit}");

    let numbers:[i32; 3] = [5, 6, 7];
    // Need mut for this: numbers[2] = 81;
    let number = numbers[2];
    println!("Number: {number}");
    
    let numbers = [7;10];
    let number = numbers[9];
    println!("Number: {number}")
}
