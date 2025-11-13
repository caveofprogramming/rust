trait Value {
    fn get(&self) -> f32;
    fn clone_box(&self) -> Box<dyn Value>;
}

#[derive(Clone)]
struct Pi {
    value: f32,
}

impl Pi {
    fn new() -> Self {
        Self { value: 3.141592 }
    }
}

impl Value for Pi {
    fn get(&self) -> f32 {
        self.value
    }

    fn clone_box(&self) -> Box<dyn Value> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
struct Surprise {
    value: f32,
}

impl Surprise {
    fn new(value: f32) -> Self {
        Self { value }
    }
}

impl Value for Surprise {
    fn get(&self) -> f32 {
        self.value
    }

    fn clone_box(&self) -> Box<dyn Value> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Value> {
    fn clone(&self) -> Box<dyn Value> {
        self.clone_box()
    }
}

fn main() {
    let values1: Vec<Box<dyn Value>> = vec![Box::new(Pi::new()), Box::new(Surprise::new(1.23))];

    for v in &values1 {
        println!("value is: {}", v.get());
    }

    println!();

    let values2 = values1.clone();

    for v in &values2 {
        println!("value is: {}", v.get());
    }
}
