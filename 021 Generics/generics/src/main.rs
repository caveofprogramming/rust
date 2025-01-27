#![allow(unused)]

fn main() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    impl Point<i32> {
        fn sum(&self)->i32 {
            self.x + self.y
        }
    }

    impl<T> Point<T> {
        fn x(&self)->&T {
            &self.x
        }
    }

    let p1 = Point { x: 1, y: 2 };
    println!("{p1:?}");
    println!("{}", p1.sum());

    let p2 = Point { x: 1.2, y: 2.3 };
    println!("{}", p2.x());

    let p3 = Point::<f32> { x: 1.2, y: 2.3 };
    println!("{p3:?}");

    println!("{}", add(7, 8));
}

fn add<T: std::ops::Add<Output = T>>(x:T, y:T)->T {
    x + y
}

