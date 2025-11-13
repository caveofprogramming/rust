#![allow(unused)]

use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Write;

fn run() -> Result<String, Error> {
    Ok("All fine.".to_string())
    //Err(Error::new(ErrorKind::Other, "Something bad happened."))
}

fn create_file(filename: String) -> Result<(), Error> {
    let mut file = File::create(filename)?;

    file.write(b"Hello there");

    File::create("test.txt")?.write(b"Hello there");

    File::open("notfound.txt")?;

    Ok(())
}

fn main() {
    let text = run().unwrap();
    println!("{text}");

    let text = run().expect("Expected string.");
    println!("{text}");

    let text = run().unwrap_or_else(|error| {
        println!("{error}");
        "Didn't work out.".to_string()
    });
    println!("{text}");

    let result = create_file("temp.txt".to_string());

    println!("{result:?}");
}
