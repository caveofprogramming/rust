#![allow(unused)]

mod details;
mod book;
use book::Book;

use details::ShowDetails;
use details::Details;


fn main() {
    

   

    let book = Book {
        author: "Charles Dickens".to_string(),
        title: "A Tale of Two Cities".to_string(),
    };

    let book2 = get_item();
    show_item(&book2);

    println!("{}", book.details());

    println!("{}", book.author);

    show_item(&book);
    show_item2(&book);
    show_item3(&book);
}

fn show_item(item: &(impl Details + ShowDetails)) {
    println!("Book details: {}", item.details());
    item.show();
}

fn show_item2<T: Details + ShowDetails>(item: &T) {
    println!("Book details: {}", item.details());
}

fn show_item3<T>(item: &T)
where
    T: Details + ShowDetails,
{
    println!("Book details: {}", item.details());
}

fn get_item()->impl Details+ShowDetails {
    Book {
        author: "Charles Dickens".to_string(),
        title: "Oliver Twist".to_string(),
    }
}
