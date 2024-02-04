// 070.rs
// https://google.github.io/comprehensive-rust/zh-CN/exercises/day-2/book-library.html

struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// Implement the methods below. Notice how the `self` parameter
// changes type to indicate the method's required level of ownership
// over the object:
//
// - `&self` for shared read-only access,
// - `&mut self` for unique and mutable access,
// - `self` for unique access by value.
impl Library {
    fn new() -> Library {
        // todo!("Initialize and return a `Library` value")
        Self { books: Vec::new() }
    }

    fn len(&self) -> usize {
        // todo!("Return the length of `self.books`")
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        // todo!("Return `true` if `self.books` is empty")
        self.books.len() == 0
    }

    fn add_book(&mut self, book: Book) {
        // todo!("Add a new book to `self.books`")
        self.books.push(book);
    }

    fn print_books(&self) {
        // todo!("Iterate over `self.books` and print each book's title and year")
        for book in &self.books {
            println!("The title is {}, and the year is {}", book.title, book.year);
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        // todo!("Return a reference to the oldest book (if any)")
        if self.books.len() == 0 { return None; }

        // expected due to this value
        // let mut oldest_book = None;
        let mut oldest_book: Option<&Book> = None;
        let mut oldest_year = 9999;
        //  `self.books` moved due to this implicit call to `.into_iter()`
        //  move occurs because `self.books` has type `Vec<Book>`, which does not implement the `Copy` trait
        // for book in self.books {

        // note: `into_iter` takes ownership of the receiver `self`, which moves `self.books`
        // for book in self.books.into_iter() {
        for book in &self.books {
            if book.year < oldest_year {
                oldest_year = book.year;
                // expected `Option<_>`, found `Book`
                // note: expected enum `Option<_>`
                // help: try wrapping the expression in `Some`
                // oldest_book = book;

                oldest_book = Some(&book);
            }
        }

        oldest_book
    }
}

fn main() {
    let mut library = Library::new();

    println!(
        "The library is empty: library.is_empty() -> {}",
        library.is_empty()
    );

    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));

    println!(
        "The library is no longer empty: library.is_empty() -> {}",
        library.is_empty()
    );

    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("The oldest book is {}", book.title),
        None => println!("The library is empty!"),
    }

    println!("The library has {} books", library.len());
    library.print_books();
}

/* result:
The library is empty: library.is_empty() -> true
The library is no longer empty: library.is_empty() -> false
The title is Lord of the Rings, and the year is 1954
The title is Alice's Adventures in Wonderland, and the year is 1865
The oldest book is Alice's Adventures in Wonderland
The library has 2 books
The title is Lord of the Rings, and the year is 1954
The title is Alice's Adventures in Wonderland, and the year is 1865
*/
