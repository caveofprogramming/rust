fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        on_holiday: bool,
    }

    impl Person {
        fn is_on_holiday(&self) -> bool {
            self.on_holiday
        }

        fn get_name(&self) -> &String {
            &self.name
        }

        fn new() -> Self {
            Self {
                name: String::from("unknown"),
                on_holiday: false,
            }
        }
    }

    let person1 = Person {
        name: String::from("Bob"),
        on_holiday: true,
    };

    let name = person1.get_name();
    let on_holiday = person1.is_on_holiday();
    println!("{name} is on holiday: {on_holiday}");
    println!("{person1:?}");

    let person2 = Person::new();
    let name = person2.get_name();
    let on_holiday = person2.is_on_holiday();
    println!("{name} is on holiday: {on_holiday}");
}
