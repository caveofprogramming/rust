#![allow(unused)]

fn main() {
    struct Star<'a> {
        x: &'a i32,
    }

    let num = 9;
    let s = Star { x: &num };
}
