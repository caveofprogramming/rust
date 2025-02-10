#![allow(unused)]

mod book;
mod items;

use book::Book;
use items::Author;
use items::Title;

fn main() {
    let book1 = get_item();

    show_author(&book1);
    show_details(&book1);
    print_author(&book1);
    print_details(&book1);

    println!("{}", get_details(&book1));
}

fn show_author(item: &impl Author) {
    println!("{}", item.author());
}

fn print_author<T: Author>(item: &T) {
    println!("{}", item.author());
}

fn show_details(item: &(impl Author + Title)) {
    println!("{} by {}", item.title(), item.author());
}

fn print_details<T: Author + Title>(item: &T) {
    println!("{} by {}", item.title(), item.author());
}

fn get_details<T>(item: &T)->String
where
    T: Author + Title,
{
    format!("{} by {}", item.title(), item.author())
}

fn get_item()->impl Author+Title {
    Book {
        author: "Charles Dickens".to_string(),
        title: "Oliver Twist".to_string(),
    }
}
