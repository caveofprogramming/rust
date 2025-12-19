use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, Condvar};

struct Fibo {
    a: i32,
    b: i32,
    result: i32,
    ready: bool,
}

impl Fibo {
    fn new() -> Self {
        Self {
            a: 0,
            b: 1,
            result: 0,
            ready: false,
        }
    }

    fn next(&mut self) {
        self.result = self.a;
        self.a = self.b;
        self.b = self.result + self.b;
    }

    fn get(&mut self) -> i32 {
        self.ready = false;
        self.result
    }
}

fn main() {
    let fb = Fibo::new();

    let fb1 = Arc::new((Mutex::new(fb), Condvar::new()));
    let fb2 = fb1.clone();

    let producer = thread::spawn(move || {

        let (lock, cond) = &*fb1;

        loop {
            if let Ok(mut fb) = lock.lock() {
                fb.next();
                fb.ready = true;
                cond.notify_one();
            }

            thread::sleep(Duration::from_millis(500));
        }
    });

    let consumer = thread::spawn(move || {

        let (lock, cond) = &*fb2;

        loop {
            if let Ok(mut fb) = lock.lock() {
                while !fb.ready {
                    fb = cond.wait(fb).unwrap();
                }
                let value = fb.get();
                println!("{value}");
            }
            
        }
    });

    let _ = producer.join();
    let _ = consumer.join();
}
