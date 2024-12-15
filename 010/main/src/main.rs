#[derive(Debug)]
struct Person {
    name: String,
    id: i32,
    is_alive: bool,
}

#[derive(Debug)]
struct Point(f64, f64, f64);

fn main() {
    let person1 = Person {
        name: String::from("Joe Bloggs"),
        id: 82,
        is_alive: true,
    };

    let person2 = Person {
        name: String::from("Mary Sue"),
        id: 93,
        ..person1
    };

    let name: String = String::from("John Smith");

    let person3 = Person {
        name,
        id: 12,
        is_alive: false,
    };

    println!("{person1:?}");
    println!("{person2:?}");
    println!("{person3:?}");

    #[derive(Debug)]
    struct Machine;
    let machine = Machine;
    println!("{machine:?}");

    let point1 = Point(0.5, 1.2, 3.4);
    println!("{point1:?}");
    displayPoint(&point1);

    display(&person1);
}

fn display(person: &Person) {
    println!(
        "Name: {}, id: {}, alive: {}",
        person.name, person.id, person.is_alive
    );
}

fn displayPoint(point: &Point) {
    println!("({}, {}, {})", point.0, point.1, point.2);
}
