use std::rc::Rc;

trait Animal {
    fn speak(&self);
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) {
        println!("I am a dog called {}", self.name);
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("I am a cat called {}", self.name);
    }
}

struct House {
    dog: Rc<Dog>,
    cat: Rc<Cat>,
    pet: Rc<dyn Animal>,
}

impl House {
    fn new() -> Self {
        let dog = Rc::new(Dog {
            name: "Fido".into(),
        });

        let cat = Rc::new(Cat {
            name: "Tiddles".into(),
        });

        let pet = Rc::clone(&dog);

        Self { dog, cat, pet }
    }

    fn activate_cat(&mut self) {
        self.pet = Rc::clone(&self.cat) as Rc<dyn Animal>;
    }

    fn speak(&self) {
        self.pet.speak();
    }
}

fn main() {
    let mut house = House::new();
    house.speak();
    house.activate_cat();
    house.speak();
}
