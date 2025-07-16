use anyhow::anyhow;
use std::env::args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::establish_connection;
    use diesel_demo::models::RoomTags;
    use diesel_demo::schema::rooms_tags::dsl::*;

    let mut conn = establish_connection();
    let method = args().nth(1);
    let rid = args().nth(2).map(|rid| rid.parse::<i32>().unwrap());
    let tags = args()
        .skip(3)
        .map(|tag| tag.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    if let Some(rid) = rid {
        if let Some(method) = method {
            if method == "add" {
                let mut insert_tags = vec![];
                for tag in tags.iter() {
                    let tag = diesel::insert_into(rooms_tags)
                        .values((room_id.eq(rid), tag_id.eq(tag)))
                        .returning(RoomTags::as_returning())
                        .get_result(&mut conn)?;
                    insert_tags.push(tag);
                }
                println!("insert room {} Tag: {:?}", rid, tags);
                return Ok(());
            } else if method == "remove" {
                let remove_tags = diesel::delete(rooms_tags)
                    .filter(room_id.eq(rid).and(tag_id.eq_any(&tags)))
                    .returning(RoomTags::as_returning())
                    .get_results(&mut conn)?;
                println!("remove room {} tag: {:?}", rid, remove_tags);
                return Ok(());
            }
        }
    }

    Err(anyhow!("A title is needed for Tag"))
}
