#![allow(unused)]

mod apple;
mod fruits;

fn main() {
    apple::speak();
    fruits::speak();
    fruits::apple::speak();
    fruits::banana::speak();
}
