use anyhow::anyhow;
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
        #[arg(short, long, help = "用户名")]
        name: String,
        #[arg(short = 'a', long, help = "头像的URL")]
        head: Option<String>,
    },
    R {
        #[arg(short, long, help = "用户id/全部")]
        id: Option<i32>,
    },
    U {
        #[arg(short, long, help = "用户id")]
        id: i32,
        #[arg(short, long, help = "用户名")]
        name: String,
        #[arg(short = 'a', long, help = "头像的URL")]
        head: Option<String>,
    },
    D {
        #[arg(short, long, help = "用户id")]
        id: i32,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::establish_connection;
    use diesel_demo::models::User;
    use diesel_demo::schema::users::dsl::*;

    let conn = &mut establish_connection();

    match Args::parse().cmd {
        Some(Command::C {
            name: input_name,
            head: input_head,
        }) => {
            let user = diesel::insert_into(users)
                .values((
                    user_name.eq(input_name),
                    input_head.map(|head_url| avatar.eq(head_url)),
                ))
                .returning(User::as_returning())
                .get_result(conn)?;
            info!("insert a User: {:?}", user);
        }
        Some(Command::U {
            id: input_id,
            name: input_name,
            head: input_head,
        }) => {
            let user = diesel::update(users.filter(id.eq(input_id)))
                .set((
                    user_name.eq(input_name),
                    input_head.map(|head_url| avatar.eq(head_url)),
                ))
                .returning(User::as_returning())
                .get_result(conn)?;
            info!("update a User: {:?}", user);
        }
        Some(Command::R { id: input_id }) => {
            let mut query = users.into_boxed();
            if let Some(input_id) = input_id {
                query = query.filter(id.eq(input_id));
            }
            let user = query
                .select(User::as_select())
                .order_by(id.asc())
                .get_results(conn)?;
            info!("read User(s): {:?}", user);
        }
        Some(Command::D { id: input_id }) => {
            let count = diesel::delete(users.filter(id.eq(input_id))).execute(conn)?;
            if count == 0 {
                return Err(anyhow!("No user found with id {}", input_id));
            }
            info!("deleted {} User(s)", count);
        }
        _ => panic!("Invalid command"),
    }
    Ok(())
}
