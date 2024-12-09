fn main() {
    run(7, 5.2);

    let x = {
        let mut y = 5;
        y += 5;
        y
    };

    println!("{x}");

    let value = 10;
    let result = add_five(value);
    println!("{value}, {result}");

    let s = String::from("Hello ");
    let s = greet(s);
    println!("{s}");
}

fn greet(mut s:String) -> String {
    s.push_str("there");›
    s
}

fn add_five(i:i32)->i32 {
    if i < 0 {
        0
    }
    else {
        i + 5
    }
}

fn run(x:i32, y:f64) -> () {
    println!("{x}, {y}");
}