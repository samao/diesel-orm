use crate::schema::{authors, books, books_authors, pages, posts};
use diesel::{prelude::*, sqlite::Sqlite};
use serde::Serialize;

// ------------------
use crate::schema::{rooms, rooms_tags, tags, users};

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Clone)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: i32,
    pub title: String,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub avatar: String,
}

#[derive(Queryable, Identifiable, Associations, Selectable, Debug, PartialEq, Serialize, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = rooms)]
pub struct Room {
    pub id: i32,
    pub title: String,
    pub is_live: bool,
    pub img_url: String,
    pub hot: i32,
    pub user_id: Option<i32>,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Room))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = rooms_tags)]
#[diesel(primary_key(room_id, tag_id))]
pub struct RoomTags {
    pub room_id: i32,
    pub tag_id: i32,
}

// ---------------

#[derive(Queryable, Selectable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(Sqlite))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Clone)]
#[diesel(table_name = books)]
pub struct Book {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct NewBook<'a> {
    pub title: &'a str,
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq, Serialize)]
#[diesel(belongs_to(Book))]
#[diesel(table_name=pages)]
pub struct Page {
    pub id: i32,
    pub page_number: i32,
    pub content: String,
    pub book_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = pages)]
pub struct NewPage {
    pub page_number: i32,
    pub content: String,
    pub book_id: i32,
}

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = authors)]
pub struct Author {
    pub id: i32,
    pub name: String,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Book))]
#[diesel(belongs_to(Author))]
#[diesel(table_name = books_authors)]
#[diesel(primary_key(book_id, author_id))]
pub struct BookAuthor {
    pub book_id: i32,
    pub author_id: i32,
}
