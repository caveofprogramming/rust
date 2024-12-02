fn main() {
    // Logical Operators
    println!("{}", true && false);
    println!("{}", true || false);
    println!("{}", !(true && false));

    // If Statements
    let temp = 3;

    if temp < 0 {
        println!("Freezing");
    }
    else if temp < 4 {
        println!("Cold");
    }
    else {
        println!("Warm");
    }

    // If Expressions
    let advice = if temp < 21 { "Take jacket" } else { "It's warm" };
    println!("{}", advice);

    // For Loop
    for count in 0..3 {
        print!("{} ", count)
    }

    println!();
    for count in 0..=3 {
        print!("{} ", count)
    }
    
    println!();
    for count in (0..3).rev() {
        print!("{} ", count)
    }
    
    // While loop
    println!();
    let mut count = 0;
    
    while count < 3 {
        count += 1;
        print!("{} ", count);
    }
    
    // Loop
    println!();
    count = 0;
    loop {
        count += 1;
        
        if count > 5 {
            break;
        }

        if count == 2 {
            continue;
        }

        print!("{} ", count);
    }

    // Labels
    println!();

    'outer: for x in 0..5 {
        for y in 0..5 {
            print!("{},{} ", x, y);

            if x*y == 6 {
                break 'outer;
            }
        }
        println!()
    }
}
