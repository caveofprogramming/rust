use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let m = m.clone();

        let handle = thread::spawn(move || {
            let mut counter = m.lock().unwrap();

            *counter += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("value: {:?}", *m.lock().unwrap());
}
