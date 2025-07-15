// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    books (id) {
        id -> Integer,
        title -> Text,
    }
}

diesel::table! {
    books_authors (book_id, author_id) {
        book_id -> Integer,
        author_id -> Integer,
    }
}

diesel::table! {
    pages (id) {
        id -> Integer,
        page_number -> Integer,
        content -> Text,
        book_id -> Integer,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    rooms (id) {
        id -> Integer,
        title -> Text,
        is_live -> Bool,
        img_url -> Text,
        hot -> Integer,
        user_id -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    rooms_tags (room_id, tag_id) {
        room_id -> Integer,
        tag_id -> Integer,
    }
}

diesel::table! {
    tags (id) {
        id -> Integer,
        title -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        user_name -> Text,
        avatar -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(books_authors -> authors (author_id));
diesel::joinable!(books_authors -> books (book_id));
diesel::joinable!(pages -> books (book_id));
diesel::joinable!(rooms -> users (user_id));
diesel::joinable!(rooms_tags -> rooms (room_id));
diesel::joinable!(rooms_tags -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    books,
    books_authors,
    pages,
    posts,
    rooms,
    rooms_tags,
    tags,
    users,
);
