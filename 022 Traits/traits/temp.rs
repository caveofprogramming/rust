#![allow(unused)]

fn main() {
    println!("Hello, world!");

    pub trait Sum {
        fn x(&self)->i32;
        fn y(&self)->i32;
        fn sum(&self) -> i32;
        fn square(&self) -> i32 {
            return i32::pow(self.x(), 2) + i32::pow(self.y(), 2);
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl Sum for Point {
        fn sum(&self) -> i32 {
            return self.x + self.y;
        }

        fn x(&self)->i32 {
            return self.x;
        }

        fn y(&self)->i32 {
            return self.y;
        }
    }

    let p1 = Point { x: 5, y: 7 };
    println!("{}", p1.sum());
    println!("{}", p1.square());
}
