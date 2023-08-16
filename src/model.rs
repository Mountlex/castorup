pub enum Entry {
    Article(Article),
    Inproceedings(Inproceedings),
}

struct Article {
    author: String,
    title: String,
    journal: String,
    year: Year,
    volume: Option<String>,
    number: Option<String>,
    pages: Option<String>,
    month: Option<Month>,
    note: Option<String>,
}

struct Inproceedings {
    author: String,
    title: String,
    booktitle: String,
    year: Year,
    editor: Option<String>,
    series: Option<String>,
    address: Option<String>,
    volume: Option<String>,
    number: Option<String>,
    pages: Option<String>,
    month: Option<Month>,
    organization: Option<String>,
    publisher: Option<String>,
    note: Option<String>,
}

struct Year(u8);
struct Month(u8);

