use clap::{Parser, Subcommand, arg};
use tracing::info;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    C {
        #[arg(short, long, help = "房间id")]
        room_id: i32,
        #[arg(short, long, help = "Tag id")]
        tag_id: i32,
    },
    R {
        #[arg(short, long, help = "房间id")]
        room_id: Option<i32>,
        #[arg(short, long, help = "Tag id")]
        tag_id: Option<i32>,
    },
    D {
        #[arg(short, long, help = "数据id")]
        room_id: Option<i32>,
        #[arg(short, long, help = "Tag id")]
        tag_id: Option<i32>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::establish_connection;
    use diesel_demo::models::RoomTags;
    use diesel_demo::schema::rooms_tags::{self, dsl::*};

    let args = Args::parse();
    let conn = &mut establish_connection();
    match args.cmd {
        Some(Command::C {
            room_id: rid,
            tag_id: tid,
        }) => {
            #[derive(Insertable)]
            #[diesel(table_name = rooms_tags)]
            struct InsertTag {
                room_id: i32,
                tag_id: i32,
            }

            let room_tag = diesel::insert_into(rooms_tags)
                .values(&InsertTag {
                    room_id: rid,
                    tag_id: tid,
                })
                .returning(RoomTags::as_returning())
                .get_result(conn)?;
            info!("create room tags: {:?}", room_tag);
        }
        Some(Command::R {
            room_id: rid,
            tag_id: tid,
        }) => {
            let mut query = rooms_tags.into_boxed();
            if let Some(rid) = rid {
                query = query.filter(room_id.eq(rid));
            }
            if let Some(tid) = tid {
                query = query.filter(tag_id.eq(tid));
            }
            let room_tags = query.select(RoomTags::as_select()).get_results(conn)?;
            info!("read room tags: {:?}", room_tags);
        }
        Some(Command::D {
            room_id: rid,
            tag_id: tid,
        }) => {
            let mut query = diesel::delete(rooms_tags).into_boxed();
            if let Some(rid) = rid {
                query = query.filter(room_id.eq(rid));
            }
            if let Some(tid) = tid {
                query = query.filter(tag_id.eq(tid));
            }
            let room_tags = query
                .returning(RoomTags::as_returning())
                .get_results(conn)?;
            info!("get room tags: {:?}", room_tags);
        }
        _ => panic!("Invalid command"),
    }

    Ok(())
}
