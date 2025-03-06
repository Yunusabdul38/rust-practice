pub struct Book<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub year: i32,
    pub likes: u16,
}

impl Book<'static> {
    pub fn new<'a>(title: &'a str, author: &'a str, year: i32) -> Book<'a> {
        Book {
            title,
            author,
            year,
            likes: 0,
        }
    }
}
