use crate::details::Author;
use crate::details::Title;

pub struct Book {
    pub author: String,
    pub title: String,
}

impl Author for Book {
    fn author(&self) -> &String {
        &self.author
    }
}

impl Title for Book {
    fn title(&self) -> &String {
        &self.title
    }
}
