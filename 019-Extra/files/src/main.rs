use std::fs::File;
use std::io::ErrorKind;

fn main() {
    //panic!("Goodbye, world!");

    

    // Open a file
    let filename = "test.txt";
    let result = File::open(filename);
    println!("Open {filename}: {result:?}");
    
    // Create a file
    let result = File::create(filename);
    println!("Create {filename}: {result:?}");

    let filename = "temp.txt";
    let result = File::open(filename);

    
    let filename = "temp.txt";

    let file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(file) => file,
                Err(error) => panic!("Unable to create {filename}"),
            }
            other => panic!("Error: {error}"),
        }
    };
}
