#![allow(unused)]

fn main() {
    println!("Hello, world!");

    pub trait Stats {
        fn x(&self) -> i32;
        fn y(&self) -> i32;
        fn sum(&self) -> i32;
        fn sum_square(&self) -> i32 {
            self.x().pow(2) + self.y().pow(2)
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl Stats for Point {
        fn x(&self) -> i32 {
            self.x
        }

        fn y(&self) -> i32 {
            self.y
        }

        fn sum(&self) -> i32 {
            self.x + self.y
        }
    }

    let p = Point { x: 5, y: 3 };
    println!("{}", p.sum());
    println!("{}", p.sum_square());
}
