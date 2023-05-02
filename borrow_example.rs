enum Gerne {
    Horror,
    Romance,
    Drama,
    Thriller,
}

struct Book {
    pages: i32,
    rating: f32,
    gerne: Gerne,
}

fn get_genre(book: &Book) -> String {
    match book.gerne {
        Gerne::Horror => format!("Horror Gerne"),
        Gerne::Romance => format!("Romace Gerne"),
        Gerne::Drama => format!("Drama Gerne"),
        Gerne::Thriller => format!("Thriller Gerne"),
    }
}

fn book_pages(book: &Book) -> i32 {
    book.pages
}

fn book_rating(book: &Book) -> f32 {
    book.rating
}

fn main() {
    let book_1 = Book {
        pages: 150,
        rating: 3.0,
        gerne: Gerne::Horror,
    };
    print!(
        "{}",
        format!(
            "{} Book has {} pages, and is rated at {} stars!! \n",
            get_genre(&book_1),
            book_pages(&book_1),
            book_rating(&book_1)
        )
    );
}
