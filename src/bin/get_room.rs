use std::collections::HashMap;

use anyhow::Ok;
use clap::{Parser, Subcommand, arg};
use diesel::{
    dsl::{max, min, sum},
    prelude::*,
};
use diesel_demo::{
    models::{Cate, Room, User},
    schema::*,
};
use serde::Serialize;
use tracing::info;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Room {
        #[arg(short, long, value_delimiter = ',', help = "房间列表id")]
        ids: Vec<i32>,
    },
    Cate {
        #[arg(short, long, value_delimiter = ',', help = "房间列表id")]
        ids: Vec<i32>,
    },
    GetHot {
        #[arg(short, long, help = "分类id")]
        id: i32,
    },
}

fn main() -> anyhow::Result<()> {
    use diesel_demo::establish_connection;
    let conn = &mut establish_connection();
    match Args::parse().cmd {
        Some(Command::Room { ids }) => {
            info!("要查看的Room IDS: {:?}", ids);
            get_rooms(conn, ids)?;
        }
        Some(Command::Cate { ids }) => {
            info!("要查看的CATE ID: {:?}", ids);
            get_cates(conn, ids.clone(), None)?;
        }
        Some(Command::GetHot { id }) => {
            info!("要查看的CATE ID: {}", id);
            get_all_hot_by_cate_id(conn, id)?;
        }
        _ => {}
    }
    dilevery_hot(conn, 1, 2, 1)?;
    Ok(())
}

fn get_all_hot_by_cate_id(conn: &mut SqliteConnection, id: i32) -> anyhow::Result<i64> {
    let data = cates::table
        .inner_join(rooms::table)
        .filter(cates::id.eq(id))
        .group_by(cates::id)
        .select((
            cates::cate_name,
            sum(rooms::hot),
            min(rooms::hot),
            max(rooms::hot),
        ))
        .first::<(String, Option<i64>, Option<i32>, Option<i32>)>(conn)?;

    info!(
        "Total hot for cate_id {}: {:?}",
        id,
        match data {
            (cate, Some(sum), Some(min), Some(max), ..) =>
                format!("{}, Sum: {:?}, Min: {}, Max: {}", cate, sum, min, max),
            _ => "No data found".to_string(),
        }
    );
    Ok(2)
}

/// 获取CATE和top3 房间
fn get_cates(
    conn: &mut SqliteConnection,
    ids: Vec<i32>,
    top_room_num: Option<u64>,
) -> anyhow::Result<()> {
    #[derive(Serialize, Debug)]
    struct FullCate {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        room: Vec<FullRoom>,
        #[serde(flatten)]
        cate: Cate,
    }

    #[derive(Serialize, Debug)]
    struct FullRoom {
        #[serde(flatten)]
        room: Room,
        user: User,
    }

    let all_cates = cates::table
        .left_join(rooms::table.on(cates::id.eq(rooms::cate_id)))
        .left_join(users::table.on(rooms::user_id.eq(users::id)))
        .filter(cates::id.eq_any(&ids))
        .order((rooms::hot.desc(), cates::id.desc()))
        .select((
            Cate::as_select(),
            Option::<Room>::as_select(),
            Option::<User>::as_select(),
        ))
        .load::<(Cate, Option<Room>, Option<User>)>(conn)?;
    let mut cate_map: HashMap<Cate, FullCate> = HashMap::with_capacity(all_cates.len());
    for (cate, room, user) in all_cates.iter() {
        let entry = cate_map.entry(cate.clone()).or_insert(FullCate {
            cate: cate.clone(),
            room: vec![],
        });

        if let Some(room) = room
            && let Some(user) = user
        {
            match top_room_num {
                Some(total) if entry.room.len() < total as usize => {
                    entry.room.push(FullRoom {
                        room: room.to_owned(),
                        user: user.to_owned(),
                    });
                }
                None => {
                    entry.room.push(FullRoom {
                        room: room.to_owned(),
                        user: user.to_owned(),
                    });
                }
                _ => {}
            }
        }
    }
    // info!("==\n{:#?}\n",all_cates.chunk_by(|a, b| a.0 == b.0));
    let cate_map = cate_map
        .into_iter()
        .map(|(_, u)| u)
        .collect::<Vec<FullCate>>();
    info!("{}", serde_json::to_string_pretty(&cate_map).unwrap());
    Ok(())
}

/// 获取房间数据
fn get_rooms(conn: &mut SqliteConnection, rids: Vec<i32>) -> anyhow::Result<()> {
    #[derive(Serialize, Debug)]
    struct FullRoom {
        #[serde(flatten)]
        room: Room,
        cate: Cate,
        user: User,
    }

    let full_str = conn.transaction(|conn| {
        type RoomWithRelations = (Room, Cate, User);
        let results: Vec<RoomWithRelations> = rooms::table
            .inner_join(cates::table.on(rooms::cate_id.eq(cates::id)))
            .inner_join(users::table.on(rooms::user_id.eq(users::id)))
            .filter(rooms::id.eq_any(&rids)) // 过滤特定房间ID
            .select((Room::as_select(), Cate::as_select(), User::as_select()))
            .load(conn)?;
        let results = results
            .into_iter()
            .map(|(room, cate, user)| FullRoom { room, cate, user })
            .collect::<Vec<FullRoom>>();
        Ok(serde_json::to_string_pretty(&results)?)
    })?;

    info!("最终查询结果:\n{}", full_str);

    Ok(())
}

/// 处理热度转移
fn dilevery_hot(conn: &mut SqliteConnection, from: i32, to: i32, amount: u8) -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::models::Room;
    use diesel_demo::schema::rooms::dsl::*;

    conn.transaction(|conn| {
        let _from_room = diesel::update(rooms)
            .filter(id.eq(from))
            .set(hot.eq(hot - amount as i32))
            .returning(Room::as_returning())
            .get_result::<Room>(conn)?;
        // println!("{:?}", from_room);

        let _to_room = diesel::update(rooms)
            .filter(id.eq(to))
            .set(hot.eq(hot + amount as i32))
            .returning(Room::as_returning())
            .get_result::<Room>(conn)?;
        // println!("{:?}", to_room);

        Ok(())
    })
}
