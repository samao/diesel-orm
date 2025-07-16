use clap::{ArgAction::SetTrue, Parser, Subcommand, arg};
use diesel::SqliteConnection;
use diesel_demo::establish_connection;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Create {
        #[arg(short, long, help = "房间标题")]
        title: String,
        #[arg(short, long, action = SetTrue, help = "是否开播标记")]
        live: bool,
        #[arg(short, long, help = "房间封面")]
        image: String,
        #[arg(short = 's', long, help = "当前热度")]
        hot: i32,
        #[arg(short, long, help = "主播id")]
        uid: Option<i32>,
        #[arg(short, long, help = "分类id")]
        cateid: Option<i32>,
    },
    Update {
        #[arg(long, short, help = "房间id")]
        id: i32,
        #[arg(short, long, help = "房间标题")]
        title: Option<String>,
        #[arg(short, long, action = SetTrue, help = "是否开播标记")]
        live: Option<bool>,
        #[arg(short = 'm', long, help = "房间封面")]
        image: Option<String>,
        #[arg(short = 's', long, help = "当前热度")]
        hot: Option<i32>,
        #[arg(short, long, help = "主播id")]
        uid: Option<i32>,
        #[arg(short, long, help = "分类id")]
        cateid: Option<i32>,
    },
    Delete {
        #[arg(short, long, help = "删除指定的房间")]
        id: i32,
    },
    Read {
        #[arg(long, short, help = "查询的房价id， 默认列出全部房间")]
        id: Option<i32>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut conn = establish_connection();

    let args = Args::parse();
    match args.cmd {
        Some(Command::Create {
            title,
            live: is_live,
            image: img_url,
            hot,
            uid: user_id,
            cateid,
        }) => {
            create_room(&mut conn, title, is_live, img_url, hot, user_id, cateid)?;
        }
        Some(Command::Read { id }) => {
            read_room(&mut conn, id)?;
        }
        Some(Command::Update {
            id,
            title,
            live,
            image,
            hot,
            uid,
            cateid,
        }) => {
            update_room(&mut conn, id, title, live, image, hot, uid, cateid)?;
        }
        Some(Command::Delete { id }) => {
            delete_room(&mut conn, id)?;
        }
        _ => {
            println!("INVALID");
        }
    }

    Ok(())
}

fn update_room(
    conn: &mut SqliteConnection,
    rid: i32,
    room_name: Option<String>,
    live: Option<bool>,
    image: Option<String>,
    hot_val: Option<i32>,
    uid: Option<i32>,
    cateid: Option<i32>,
) -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::models::{Room, RoomUpdate};
    use diesel_demo::schema::rooms::dsl::*;
    let room_update = RoomUpdate {
        title: room_name.clone(),
        is_live: live,
        img_url: image.clone(),
        hot: hot_val,
        user_id: uid,
        cate_id: cateid,
    };
    let room = diesel::update(rooms.filter(id.eq(rid)))
        .set(&room_update)
        .returning(Room::as_returning())
        .get_result(conn)?;
    println!("update room: {:?}", room);
    Ok(())
}

fn delete_room(conn: &mut SqliteConnection, rid: i32) -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::models::Room;
    use diesel_demo::schema::rooms::dsl::*;
    let dels = diesel::delete(rooms)
        .filter(id.eq(rid))
        .returning(Room::as_returning())
        .get_result(conn)?;
    println!("delete room: {:?}", dels);
    Ok(())
}

fn read_room(conn: &mut SqliteConnection, rid: Option<i32>) -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::models::Room;
    use diesel_demo::schema::rooms::dsl::*;
    let mut query = rooms.into_boxed();
    if let Some(rid) = rid {
        query = query.filter(id.eq(rid));
    }
    let room = query.select(Room::as_returning()).get_results(conn)?;
    println!("read room: {:?}", room);
    Ok(())
}

fn create_room(
    conn: &mut SqliteConnection,
    name: String,
    live: bool,
    img: String,
    hot_val: i32,
    user: Option<i32>,
    cateid: Option<i32>,
) -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::models::Room;
    use diesel_demo::schema::rooms::dsl::*;

    let room = diesel::insert_into(rooms)
        .values((
            title.eq(name),
            is_live.eq(live),
            img_url.eq(img),
            hot.eq(hot_val),
            user_id.eq(user),
            cate_id.eq(cateid),
        ))
        .returning(Room::as_returning())
        .get_result(conn)?;

    println!("创建新房间: {:?}", room);

    Ok(())
}
