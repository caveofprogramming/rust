#![allow(unused)]

fn main() {
    let x;

    {
        let y = 8;
        x = &y;
    }
    println!("{x}");

}
