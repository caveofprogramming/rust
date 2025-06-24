use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {

    let value = 0;

    let h1 = thread::spawn(|| {
        for i in 0..5 {
            println!("A: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    let h2 = thread::spawn(|| {
        for i in 0..5 {
            println!("B: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    h1.join().unwrap();
    h2.join().unwrap();
}
