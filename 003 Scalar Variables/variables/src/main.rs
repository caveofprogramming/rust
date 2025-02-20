fn main() {
    const PI:f64 = 3.141592;

    println!("PI is {PI}");

    let x = 10;
    let x = 9;  // Shadowing

    // Note, can't do this: x = 1.23;
    // Since x is now an integer type, can't assign non-integer values.

    println!("x is {x}");

    let mut y = 20;
    y = 15;
    println!("y is {y}");

    let value1:i8 = -12;
    println!("value1 is {value1}");

    // Prefixed with underscore because unused.
    let _value2:i16 = 12;
    let _value3:i32 = 12;
    let _value4:i64 = 12;
    let _value5:i128 = 12;
    let _value6:bool = true;
    let _value7:char = 'c';
    let _value7:isize = 54;

    let _value8:f32 = 54.0;
    let _value9:f64 = 54.0;

    let _value10:u16 = 12;
    let _value11:usize = 54;

    let letter:char = 'A';
    let flag:bool = true;


}
