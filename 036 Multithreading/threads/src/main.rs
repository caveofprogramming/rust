use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();

    let producer1 = thread::spawn(move || {
        for i in 0..5 {
            tx1.send(i).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    let producer2 = thread::spawn(move || {
        for i in (5..10).rev() {
            tx2.send(i).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    let receiver = thread::spawn(move || {
        for v in rx {
            println!("thread: {}", v);
        }
    });

    receiver.join().unwrap();
    producer1.join().unwrap();
    producer2.join().unwrap();
}
