fn main() {
    
    let greet = || println!("Hello");
    greet();

    let add = |a,b| a+b;
    println!("Result: {}", add(5, 6));
    
    let multiply = |a:i32,b:i32|->i32 { a*b };
    println!("Result: {}", multiply(5, 6));

    run_op(add);
    run_op(multiply);

    let show = |a| println!("{a}");
    show(5);
    //show(5.1); // This causes an error.

    let values = vec![1, 2, 3];
    let show = || println!("{values:?}");
    show();

    let mut values = vec![1, 2, 3];
    let mut push = || values.push(4);
    push();
    println!("{values:?}");
}

fn run_op(f: fn(i32,i32)->i32) {
    println!("Result: {}", f(5, 6));
}
