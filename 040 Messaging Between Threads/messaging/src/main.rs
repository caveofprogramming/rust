use crossbeam_channel::{Receiver, Sender, bounded, unbounded};
use std::thread;
use std::time::Duration;

enum Message {
    Terminate,
    GetStatus { id: u32, reply_to: Sender<String> },
}

fn main() {
    let (tx, rx): (Sender<Message>, Receiver<Message>) = unbounded();

    let (tx1, rx1) = (tx.clone(), rx.clone());

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(3000));

        let (reply_tx, reply_rx): (Sender<String>, Receiver<String>) =
            bounded(1);

        tx1.send(Message::GetStatus {
            id: 123,
            reply_to: reply_tx,
        }).unwrap();

        let message = reply_rx.recv().unwrap();
        println!("Received message: {}", message);

        thread::sleep(Duration::from_millis(3000));

        tx1.send(Message::Terminate).unwrap();
    });

    let receiver_th = thread::spawn(move || {
        while let Ok(message) = rx1.recv() {
            match message {
                Message::Terminate => {
                    println!("Terminate message received");
                    break;
                }
                Message::GetStatus { id, reply_to } => {
                    reply_to.send(format!("Everything OK with {}", id)).unwrap();
                }
            }
        }
    });

    receiver_th.join().unwrap();
}
