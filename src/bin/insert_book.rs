use std::io::stdin;

use self::models::Book;
use diesel::prelude::*;
use diesel_demo::{models::NewBook, *};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use self::schema::books;
    let conn = &mut establish_connection();
    let mut title = String::new();
    println!("What would you like your title to be?");
    stdin().read_line(&mut title)?;
    let title = title.trim_end();

    let new_book = NewBook { title };
    let book = diesel::insert_into(books::table)
        .values(&new_book)
        .returning(Book::as_returning())
        .get_result(conn)?;
    println!("\nOk! Saved draft {} with id {}", title, book.id);
    Ok(())
}
