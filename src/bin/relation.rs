use diesel::prelude::*;
use diesel_demo::{
    establish_connection,
    models::{Author, Book, BookAuthor, Page},
    schema::{authors, books, books_authors, pages},
};
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;
    let mut conn = establish_connection();

    let inited = diesel::select(diesel::dsl::exists(
        books::table.filter(books::title.eq("javascript std")),
    ))
    .get_result::<bool>(&mut conn)?;

    if !inited {
        println!("初始化数据");
        setup_data(&mut conn)?;
    } else {
        println!("跳过数据初始化");
    }

    one_to_n_relations(&mut conn)?;
    joins(&mut conn)?;
    m_to_n_relations(&mut conn)?;

    Ok(())
}

fn m_to_n_relations(conn: &mut SqliteConnection) -> anyhow::Result<()> {
    let libo = authors::table
        .filter(authors::name.eq("libo"))
        .select(Author::as_select())
        .get_result(conn)?;
    let books = BookAuthor::belonging_to(&libo)
        .inner_join(books::table)
        .select(Book::as_select())
        .load(conn)?;
    println!("libo books: {:?}", books);
    let book = books::table
        .filter(books::title.eq("ai in 2025"))
        .select(Book::as_select())
        .get_result(conn)?;
    let authors = BookAuthor::belonging_to(&book)
        .inner_join(authors::table)
        .select(Author::as_select())
        .load(conn)?;
    println!(r#"Authors for "{}": {:?}"#, book.title, authors);

    let all_authors = authors::table.select(Author::as_select()).load(conn)?;
    let books = BookAuthor::belonging_to(&authors)
        .inner_join(books::table)
        .select((BookAuthor::as_select(), Book::as_select()))
        .load(conn)?;

    let books_per_author = books
        .grouped_by(&all_authors)
        .into_iter()
        .zip(authors)
        .map(|(b, author)| (author, b.into_iter().map(|(_, book)| book).collect()))
        .collect::<Vec<(Author, Vec<Book>)>>();
    println!("All authors including their books: {books_per_author:?}");

    Ok(())
}

fn joins(conn: &mut SqliteConnection) -> anyhow::Result<()> {
    let page_with_book = pages::table
        .inner_join(books::table)
        .filter(books::title.eq("javascript std"))
        .select((Page::as_select(), Book::as_select()))
        .load::<(Page, Book)>(conn)?;
    println!("Page-Book pairs:{:?}", page_with_book);
    let book_option_pages = books::table
        .left_join(pages::table)
        .select((Book::as_select(), Option::<Page>::as_select()))
        .load::<(Book, Option<Page>)>(conn)?;
    println!(
        "Book-Page pairs (including empty books): {:?}",
        book_option_pages
    );
    Ok(())
}

fn one_to_n_relations(conn: &mut SqliteConnection) -> anyhow::Result<()> {
    let java_std = books::table
        .filter(books::title.like("%javascript std%"))
        .select(Book::as_select())
        .get_result(conn)?;

    let pages = Page::belonging_to(&java_std)
        .select(Page::as_select())
        .load(conn)?;
    println!(r#"Pages for "{}":\n{:?}"#, java_std.title, pages);

    let all_books = books::table.select(Book::as_select()).load(conn)?;
    let pages = Page::belonging_to(&all_books)
        .select(Page::as_select())
        .load(conn)?;

    let pages_per_book = pages
        .grouped_by(&all_books)
        .into_iter()
        .zip(all_books)
        .map(|(pages, book)| (book, pages))
        .collect::<Vec<(Book, Vec<Page>)>>();
    println!("Pages per book:\n{:?}\n", pages_per_book);
    Ok(())
}

fn setup_data(conn: &mut SqliteConnection) -> anyhow::Result<()> {
    let book = create_book(conn, "javascript std")?;
    create_page(conn, 1, &book, "这是第一个页面")?;
    create_page(conn, 1, &book, "This is second page")?;
    let libo = create_author(conn, "libo")?;
    concat_book_and_author(conn, &book, &libo)?;

    let liya = create_author(conn, "liya")?;
    let rust_cookbook = create_book(conn, "rust cook book")?;
    concat_book_and_author(conn, &rust_cookbook, &liya)?;

    let ai_edition = create_book(conn, "ai in 2025")?;
    concat_book_and_author(conn, &ai_edition, &libo)?;
    concat_book_and_author(conn, &ai_edition, &liya)?;
    Ok(())
}

fn create_book(conn: &mut SqliteConnection, title: &str) -> anyhow::Result<Book> {
    let book = diesel::insert_into(books::table)
        .values(books::title.eq(title))
        .returning(Book::as_returning())
        .get_result(conn)?;
    Ok(book)
}

fn create_page(
    conn: &mut SqliteConnection,
    pn: i32,
    book: &Book,
    content: &str,
) -> anyhow::Result<Page> {
    let page = diesel::insert_into(pages::table)
        .values((
            pages::page_number.eq(pn),
            pages::book_id.eq(book.id),
            pages::content.eq(content),
        ))
        .returning(Page::as_returning())
        .get_result(conn)?;

    Ok(page)
}

fn create_author(conn: &mut SqliteConnection, name: &str) -> anyhow::Result<Author> {
    let author = diesel::insert_into(authors::table)
        .values(authors::name.eq(name))
        .returning(Author::as_returning())
        .get_result(conn)?;
    Ok(author)
}

fn concat_book_and_author(
    conn: &mut SqliteConnection,
    book: &Book,
    author: &Author,
) -> anyhow::Result<BookAuthor> {
    let book_author = diesel::insert_into(books_authors::table)
        .values((
            books_authors::book_id.eq(book.id),
            books_authors::author_id.eq(author.id),
        ))
        .returning(BookAuthor::as_returning())
        .get_result(conn)?;

    Ok(book_author)
}
