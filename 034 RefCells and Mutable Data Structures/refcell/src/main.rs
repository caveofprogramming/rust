#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;

type DataRef = Rc<RefCell<Data>>;

struct Data {
    name: String,
}

impl Data {
    fn new(name: impl Into<String>) -> DataRef {
        Rc::new(RefCell::new(Data { name: name.into() }))
    }

    fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        println!("dropped {}", self.name);
    }
}

struct Connector {
    left: DataRef,
    right: DataRef,
}

impl Connector {
    fn new(left: DataRef, right: DataRef) -> Connector {
        Connector {
            left: left.clone(),
            right: right.clone(),
        }
    }

    fn left(&self) -> DataRef {
        self.left.clone()
    }

    fn right(&self) -> DataRef {
        self.right.clone()
    }

    fn print(&self) {
        println!("{}-{}", self.left.borrow().name, self.right.borrow().name);
    }
}

fn main() {
    let d1 = Data::new("B");
    let d2 = Data::new("U");
    let d3 = Data::new("G");

    let c1 = Connector::new(d1, d2);
    let c2 = Connector::new(c1.right(), d3);
    let c3 = Connector::new(c2.right(), c1.left());

    c1.print();
    c2.print();
    c3.print();

    c2.right().borrow_mut().set_name("I");

    println!();

    c1.print();
    c2.print();
    c3.print();
}
