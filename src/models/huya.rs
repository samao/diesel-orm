use crate::schema::{rooms, rooms_tags, tags, users};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Clone)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: i32,
    pub title: String,
}
