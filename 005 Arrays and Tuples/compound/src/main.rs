// Rust creator: Graydon Hoare

fn main() {
    let mut numbers = [1, 2, 3];
    println!("{:?}", numbers);
    
    numbers[0] = 7;
    println!("{:?}", numbers);

    for i in numbers {
        println!("{i}");
    }

    let numbers:[u32; 2] = [6, 7];
    println!("{:?}", numbers);
    
    let numbers = [7; 3];
    println!("{:?}", numbers);
    
    let mut values = ("hello", 7);
    println!("{:?}", values);
    
    let x = values.0;
    println!("{x}");
    
    values.1 = 8;
    println!("{:?}", values);
    
    let mut values:(u32, f64);
    values = (0, 0.1);
    values.0 = 9;
    values.1 = 1.2;
    println!("{:?}", values);
}
