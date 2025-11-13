#![allow(unused)]

mod book;
mod details;

use book::Book;
use details::Author;
use details::Title;

fn main() {
    let book = get_author();

    show_author(&book);
    show_details(&book);
    print_author(&book);
    print_details(&book);
    show(&book);
}

fn show_author(item: &impl Author) {
    println!("{}", item.author());
}

fn print_author<T: Author>(item: &T) {
    println!("{}", item.author());
}

fn print_details<T: Author + Title>(item: &T) {
    println!("{} by {}", item.title(), item.author());
}

fn show<T>(item: &T)
where
    T: Author + Title,
{
    println!("Book: {} by {}", item.title(), item.author());
}

fn show_details(item: &(impl Author + Title)) {
    println!("{} by {}", item.title(), item.author());
}

fn get_author() -> impl Author + Title {
    Book {
        author: "Charles Dickens".to_string(),
        title: "A Tale of Two Cities".to_string(),
    }
}
