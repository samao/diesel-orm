use anyhow::anyhow;
use std::env::args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::establish_connection;
    use diesel_demo::models::Tag;
    use diesel_demo::schema::tags::dsl::*;

    let mut conn = establish_connection();
    let method = args().nth(1);
    let tag_title = args().nth(2);

    if let Some(method) = method
        && let Some(tag_title) = tag_title
    {
        if method == "add" {
            let tag = diesel::insert_into(tags)
                .values(title.eq(tag_title))
                .returning(Tag::as_returning())
                .get_result(&mut conn)?;
            println!("insert a Tag: {:?}", tag);
            return Ok(());
        } else if method == "remove" {
            let tag_id = diesel::delete(tags)
                .filter(title.eq(tag_title))
                .returning(Tag::as_returning())
                .get_result(&mut conn)?;
            println!("remove tag: {:?}", tag_id);
            return Ok(());
        }
    }

    Err(anyhow!("A title is needed for Tag"))
}
