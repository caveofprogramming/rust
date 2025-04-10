#![allow(unused)]

use std::env;
use std::process;

fn main() {
    let args = parse_args(env::args()).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    println!("{} is {}", args.name, args.age);
}

fn parse_args(mut args:impl Iterator<Item=String>)->Result<Args, &'static str> {

    args.next();

    let name = match args.next() {
        Some(name) => name,
        None => return Err("Name argument not supplied."),
    };

    let age = args
        .next()
        .ok_or("Age argument not supplied")?
        .parse::<u32>()
        .map_err(|_| "Invalid age supplied")?;

    Ok(Args{name, age})
}

struct Args {
    name: String,
    age: u32,
}

