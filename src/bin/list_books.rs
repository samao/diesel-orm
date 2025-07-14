use self::models::Book;
use diesel::prelude::*;
use diesel_demo::{models::Page, *};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct BookWithPages {
    #[serde(flatten)]
    book: Book,
    pages: Vec<Page>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use self::schema::books::dsl::books;
    let conn = &mut establish_connection();

    let all_books = books.select(Book::as_select()).load(conn)?;

    let pages = Page::belonging_to(&all_books)
        .select(Page::as_select())
        .load(conn)?;
    let _pages_per_book = dbg!(
        dbg!(
            dbg!(
                dbg!(dbg!(pages).grouped_by(dbg!(&all_books)))
                    .into_iter()
                    .zip(all_books)
            )
            .map(|(pages, book)| BookWithPages { book, pages })
        )
        .collect::<Vec<BookWithPages>>()
    );

    // println!("Pages per book:\n {pages_per_book:?}\n");
    Ok(())
}
