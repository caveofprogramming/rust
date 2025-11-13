fn main() {
    let value = Some(99);
    println!("{value:?}");

    let mut value: Option<String> = Option::None;
    value = Some(String::from("Greeting"));
    println!("{value:?}");



    /*
    #[derive(Debug)]
    enum StarType {
        RedDwarf = 0,
        YellowDwarf = 7,
        RedGiant = 10,
    }

    let star: StarType = StarType::RedGiant;
    println!("{star:?}");

    #[derive(Debug)]
    enum Entity {
        Star(String, StarType),
        Planet(String),
    }

    let sun: Entity = Entity::Star(String::from("The Sun"), StarType::YellowDwarf);
    println!("{sun:?}");

    let value = Some(99);
    println!("{value:?}");

    let mut value:Option<String> = None;
    value = Some(String::from("Greeting"));
    println!("{value:?}");
     */
}
