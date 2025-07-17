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
        #[arg(short, long, help = "Tag的标题")]
        title: String,
    },
    R {
        #[arg(short, long, help = "数据id")]
        id: Option<i32>,
    },
    U {
        #[arg(short, long, help = "数据id")]
        id: i32,
        #[arg(short, long, help = "Tag的标题")]
        title: String,
    },
    D {
        #[arg(short, long, help = "数据id")]
        id: i32,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::establish_connection;
    use diesel_demo::models::Tag;
    use diesel_demo::schema::tags::dsl::*;

    let args = Args::parse();
    let conn = &mut establish_connection();
    match args.cmd {
        Some(Command::C { title: input_tag }) => {
            let tag = diesel::insert_into(tags)
                .values(title.eq(input_tag))
                .returning(Tag::as_returning())
                .get_result(conn)?;
            info!("insert a Tag: {:?}", tag);
        }
        Some(Command::U {
            id: input_id,
            title: input_tag,
        }) => {
            let tag = diesel::update(tags.filter(id.eq(input_id)))
                .set(title.eq(input_tag))
                .returning(Tag::as_returning())
                .get_result(conn)?;
            info!("update a Tag: {:?}", tag);
        }
        Some(Command::R { id: input_id }) => {
            let mut query = tags.into_boxed();
            if let Some(input_id) = input_id {
                query = query.filter(id.eq(input_id));
            }
            let tag_list: Vec<Tag> = query.select(Tag::as_select()).order_by(id.asc()).get_results(conn)?;
            info!("query tag list: {:?}", tag_list);
        }
        Some(Command::D { id: input_id }) => {
            let tag_id = diesel::delete(tags.filter(id.eq(input_id)))
                .returning(Tag::as_returning())
                .get_result(conn)?;
            info!("remove tag: {:?}", tag_id);
        }
        _ => panic!("Invalid command"),
    }

    Ok(())
}
