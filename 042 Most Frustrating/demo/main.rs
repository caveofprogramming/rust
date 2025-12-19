use std::thread;
use std::sync::{Mutex, Arc, Condvar};
use std::time::Duration;

struct Producer {
    a: i32,
    b: i32,
    result: i32,
}

impl Producer {
    fn new() -> Self {
        Self {
            a: 0,
            b: 1,
            result: 0,
        }
    }

    fn next(&mut self) {
        self.result = self.a;
        self.a = self.b;
        self.b = self.result + self.b;
    }

    fn get(&self) -> i32 {
        self.result
    }
}

fn main() {
    let fibo = Arc::new((Mutex::new(Producer::new()), Condvar::new()));

    let fibo1 = fibo.clone();

    let producer = thread::spawn(move ||{
        let (lock, cvar) = &*fibo1;
        loop {
            if let Ok(mut g) = lock.lock() {
                g.next();
                cvar.notify_one();
            }
            thread::sleep(Duration::from_millis(500));
        }
    });

    let consumer = thread::spawn(move ||{
        let (lock, cvar) = &*fibo;
        loop {
            if let Ok(g) = lock.lock() {
                let g = cvar.wait(g).unwrap();
                let result = g.get();
                println!("{result} ");
            }
            thread::sleep(Duration::from_millis(500));
        }
    });

    let _ = producer.join();
    let _ = consumer.join();
}
