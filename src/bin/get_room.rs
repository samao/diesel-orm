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

    // let room_with_cate = rooms::table
    //     .inner_join(cates::table)
    //     .filter(cates::id.eq(8))
    //     .select((Room::as_select(), Cate::as_select()))
    //     .load::<(Room, Cate)>(conn)?;

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
    Ok(())
}
