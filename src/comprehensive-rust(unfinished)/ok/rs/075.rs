// 075.rs
// https://google.github.io/comprehensive-rust/zh-CN/std/hashmap.html

use std::collections::HashMap;

fn main() {
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of Huckleberry Finn".to_string(), 207);
    page_counts.insert("Grimms' Fairy Tales".to_string(), 751);
    page_counts.insert("Pride and Prejudice".to_string(), 303);

    if !page_counts.contains_key("Les Misérables") {
        println!("We know about {} books, but not Les Misérables.",
                 page_counts.len());
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown.")
        }
    }

    // Use the .entry() method to insert a value if nothing is found.
    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        let page_count: &mut i32 = page_counts.entry(book.to_string()).or_insert(0);
        *page_count += 1;
    }

    println!("{page_counts:#?}");
}

/* result:
We know about 3 books, but not Les Misérables.
Pride and Prejudice: 303 pages
Alice's Adventure in Wonderland is unknown.
{
    "Grimms' Fairy Tales": 751,
    "Adventures of Huckleberry Finn": 207,
    "Alice's Adventure in Wonderland": 1,
    "Pride and Prejudice": 304,
}
*/
