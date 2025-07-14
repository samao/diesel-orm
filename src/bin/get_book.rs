use std::env::args;

use self::models::Book;
use diesel::prelude::*;
use diesel_demo::{
    models::Page,
    schema::{
        books::{table, title},
        pages,
    },
    *,
};

fn main() -> anyhow::Result<()> {
    use self::schema::books::dsl::books;

    let book_id = args()
        .nth(1)
        .expect("get_post requires a posst id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();
    let post = books
        .find(book_id)
        .select(Book::as_select())
        .first(connection)
        .optional();
    match post {
        Ok(Some(post)) => {
            let pages = Page::belonging_to(&post)
                .select(Page::as_select())
                .load(connection)
                .unwrap_or_default();
            println!(
                "Post with id: {} has a title: <{}>, pages: {:?}",
                post.id, post.title, pages
            );
        }
        Ok(None) => println!("Unable to find post {}", book_id),
        _ => println!("An error occured while fetching post {}", book_id),
    }

    let page_with_book = pages::table
        .inner_join(table)
        .filter(title.eq("rust helper"))
        .select((Page::as_select(), Book::as_select()))
        .load::<(Page, Book)>(connection)?;

    println!("PAGE-BOOK pairs: {:#?}", page_with_book);

    let book_without_pages = table
        .left_join(pages::table)
        .select((Book::as_select(), Option::<Page>::as_select()))
        .load::<(Book, Option<Page>)>(connection)?;

    println!("Book-Page without pairs (including empty books): {book_without_pages:#?}");

    Ok(())
}
