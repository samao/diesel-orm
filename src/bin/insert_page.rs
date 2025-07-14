use std::env::args;

use self::models::Page;
use diesel::prelude::*;
use diesel_demo::{models::NewPage, *};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use self::schema::pages;
    let conn = &mut establish_connection();
    let book_id = args().nth(1).expect("need page id").parse::<i32>()?;
    let page_number = args()
        .nth(2)
        .expect("you should set page number.")
        .parse::<i32>()?;
    let content = args().nth(3).expect("Invalid Content");

    let new_page = NewPage {
        book_id,
        page_number,
        content,
    };

    let page = diesel::insert_into(pages::table)
        .values(&new_page)
        .returning(Page::as_returning())
        .get_result(conn)?;

    println!(
        "\nOk! Saved page draft {} book id {} num {}",
        page.id, page.book_id, page.page_number
    );
    Ok(())
}
