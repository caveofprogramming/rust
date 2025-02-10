
mod details;

use details::Details;
use details::ShowDetails;

pub struct Book {
    title: String,
    author: String,
}

pub impl Details for Book {
    fn details(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

pub impl ShowDetails for Book {
    fn show(&self) {
        println!("The book {} is by {}", self.title, self.author);
    }
}