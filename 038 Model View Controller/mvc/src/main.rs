/*
 * Model struct which contains one or more vectors
 * View struct which can present one of these vectors
 * App creates the model and one or more views
 */

use std::cell::RefCell;
use std::rc::Rc;

type Data = Rc<RefCell<Vec<i32>>>;

struct Model {
    data1: Data,
    data2: Data,
}

impl Model {
    fn new() -> Self {
        Self {
            data1: Rc::new(RefCell::new(vec![0; 10])),
            data2: Rc::new(RefCell::new(vec![1; 10])),
        }
    }
}

struct View {
    data: Data,
}

impl View {
    fn new(data: Data) -> Self {
        data.borrow_mut()[0] = 7;
        Self { data }
    }

    fn print(&self) {
        for i in self.data.borrow().iter() {
            print!("{} ", i);
        }
        println!();
    }
}

struct App {
    model: Rc<RefCell<Model>>,
    view: View,
}

impl App {
    fn new() -> Self {
        let model = Rc::new(RefCell::new(Model::new()));

        Self {
            model: Rc::clone(&model),
            view: View::new(Rc::clone(&model.borrow().data1)),
        }
    }

    fn print(&self) {
        self.view.print();
    }
}

fn main() {
    let app = App::new();
    app.print();
}
