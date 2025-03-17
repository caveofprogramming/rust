fn main() {
    let greet = || println!("Hello");
    greet();

    let add = |a, b| a + b;
    let result = add(5, 2);
    println!("{result}");

    let add = |a:i32, b:i32|->i32 { a + b };
    let result = add(7, 9);
    println!("{result}");

    let add:fn(i32,i32)->i32 = |a, b| a + b;
    let result = add(1, 4);
    println!("{result}");

    let add = |a, b| a + b;
    let result = add(5.1, 2.2);
    println!("{result}");
    //let result = add(5, 2);

    let values = vec![1, 2, 3];
    let show = || println!("{values:?}");
    show();

    let mut values = vec![1, 2, 3];
    let mut show = || values.push(4);
    show();
    println!("{values:?}");

    let result = run_op(|a,b| a*b, 5, 4);
    println!("{result}");
}

fn run_op(op:fn(i32,i32)->i32, a:i32, b:i32)->i32 {
    op(a, b)
}
