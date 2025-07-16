use diesel::SqliteConnection;
use diesel_demo::schema::rooms::hot;

fn main() -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::{
        establish_connection,
        models::{Cate, Room},
        schema::*,
    };
    use serde::Serialize;

    let conn = &mut establish_connection();

    #[derive(Serialize, Debug)]
    struct CateWithRooms {
        #[serde(flatten)]
        cate: Cate,
        rooms: Vec<Room>,
    }

    let cate = cates::table
        .filter(cates::id.eq(8))
        .select(Cate::as_select())
        .get_results(conn)?;
    // println!("cate: {:?}", cate);
    let cate_rooms = Room::belonging_to(&cate)
        .select(Room::as_select())
        .limit(10)
        .order_by(hot.desc())
        .load::<Room>(conn)?;
    let rooms_per_cate = cate_rooms
        .grouped_by(&cate)
        .into_iter()
        .zip(cate)
        .map(|(rooms, cate)| CateWithRooms { cate, rooms })
        .collect::<Vec<CateWithRooms>>();
    println!("randcate:{:?}", serde_json::to_string(&rooms_per_cate)?);

    dilevery_hot(conn, 1, 2, 10)?;
    Ok(())
}

fn dilevery_hot(conn: &mut SqliteConnection, from: i32, to: i32, amount: u8) -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::models::Room;
    use diesel_demo::schema::rooms::dsl::*;

    conn.transaction(|conn| {
        let from_room = diesel::update(rooms)
            .filter(id.eq(from))
            .set(hot.eq(hot - amount as i32))
            .returning(Room::as_returning())
            .get_result::<Room>(conn)?;
        println!("{:?}", from_room);

        let to_room = diesel::update(rooms)
            .filter(id.eq(to))
            .set(hot.eq(hot + amount as i32))
            .returning(Room::as_returning())
            .get_result::<Room>(conn)?;
        println!("{:?}", to_room);

        Ok(())
    })
}
