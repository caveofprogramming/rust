use std::sync::{Arc, RwLock};
use std::thread;


struct Model {
    data1: Arc<RwLock<Vec<i32>>>,
    data2: Arc<RwLock<Vec<i32>>>,
}

impl Model {
    fn new() -> Self {
        Self {
            data1: Arc::new(RwLock::new(vec![0; 10])),
            data2: Arc::new(RwLock::new(vec![1; 10])),
        }
    }
}

struct View {
    data: Arc<RwLock<Vec<i32>>>,
}

impl View {
    fn new(data: Arc<RwLock<Vec<i32>>>) -> Self {
        Self { data }
    }
}

struct Controller {
    model: Arc<RwLock<Model>>,
    view1: View,
    view2: View,
}

struct Engine {
    model: Arc<RwLock<Model>>,
}

impl Controller {
    fn new() -> Self {

        let model = Arc::new(RwLock::new(Model::new()));

        Self {
            model: Arc::clone(&model),
            view1: View::new(Arc::clone(&model.data1)),
            view2: View::new(Arc::clone(&model.data2)),
        }
    }
}

fn main() {
    let model = Arc::new(RwLock::new(Model::new()));
}
