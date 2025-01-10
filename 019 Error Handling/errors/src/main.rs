#![allow(unused)]
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //panic!("oh no");

    let mut values = vec![0, 1, 2];
    let x = &mut values[2];
    *x += 1;
    println!("{x}");
    println!("{values:?}");

    // values[3];
    let filename = "test.txt";
    let result = File::open(filename);
    println!("Open {filename}: {result:?}");

    let file = match result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fh) => fh,
                Err(e) => panic!("Cannot create file {filename}."),
            },
            other => {
                panic!("{other:?}");
            }
        },
    };
}
