use crate::items::Author;
use crate::items::Title;

pub struct Book {
    pub title: String,
    pub author: String,
}

impl Title for Book {
    fn title(&self) -> &String {
        &self.title
    }
}

impl Author for Book {
    fn author(&self) -> &String {
        &self.author
    }
}
