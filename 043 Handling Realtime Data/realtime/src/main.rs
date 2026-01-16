/*
 * Producer-Consumer
 * - Feed buffered data in real time to a front end.
 * - Front end cannot have any glitches
 *   so cannot use locks.
 */

use crossbeam::channel::{Receiver, Sender, bounded};
use std::thread;
use std::time::Duration;

const BLOCK_SIZE: usize = 1;
const N_BLOCKS: usize = 4;
type BLOCK = Box<[f64; BLOCK_SIZE]>;

fn main() {
    let (tx_to_consumer, rx_from_producer): (Sender<BLOCK>, Receiver<BLOCK>) = bounded(N_BLOCKS);
    let (tx_to_producer, rx_from_consumer): (Sender<BLOCK>, Receiver<BLOCK>) = bounded(N_BLOCKS);

    let producer = thread::spawn(move || {
        for i in 0..8 {
            if i < N_BLOCKS {
                if tx_to_consumer
                    .send(Box::new([(i + 1) as f64; BLOCK_SIZE]))
                    .is_err()
                {
                    break;
                }

                println!("producer: sent block {}", i + 1);
            } else {
                if let Ok(mut block) = rx_from_consumer.recv() {
                    block[0] = (i + 1) as f64;

                    if tx_to_consumer.send(block).is_err() {
                        break;
                    }

                    println!("producer: sent block (recycled) {}", i + 1);
                }
            }
        }
    });

    let consumer = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(500));

            if let Ok(block) = rx_from_producer.try_recv() {
                // (Do something with the block)
                println!("consumer: {block:?}");

                if tx_to_producer.send(block).is_err() {
                    println!("consumer: producer terminated");
                }
            } else {
                break;
            }
        }
    });

    let _ = producer.join();
    let _ = consumer.join();
}
