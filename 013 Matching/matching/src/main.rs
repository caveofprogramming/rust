#![allow(unused)]

fn main() {
    let number1 = Some(5);
    let number2: Option<i32> = None;

    let result = match number1 {
        Some(value) => value,
        None => 0,
        // _ => 0,
    };

    println!("{result:?}");

    #[derive(Debug)]
    enum Point {
        Point1D(i32),
        Point2D(i32, i32),
        Point3D(i32, i32, i32),
    }

    let mut point = Point::Point2D(21, 22);
    point = Point::Point1D(33);
    point = Point::Point3D(41, 42, 43);

    fn print_point(point: &Point) {
        println!("{point:?}");
    }

    match point {
        Point::Point1D(c1) => {
            println!("Point1D: {c1:?}");
        }
        Point::Point2D(c1, c2) => {
            println!("Point2D: {c1:?}, {c2:?}");
        }
        other => print_point(&other),
    }
}
