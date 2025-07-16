use anyhow::anyhow;
use clap::{Parser, Subcommand, arg};
use diesel::SqliteConnection;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Add {
        #[arg(short, long)]
        icon: String,
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        total: i32,
    },
    Delete {
        #[arg(short, long)]
        id: i32,
    },
    GET,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use diesel_demo::establish_connection;
    let conn = &mut establish_connection();
    let args = Args::parse();
    match args.cmd {
        Some(Command::Add { icon, name, total }) => {
            create_cate(conn, icon, name, total)?;
        }
        Some(Command::Delete { id }) => {
            delete_cate(conn, id)?;
        }
        Some(Command::GET) => {
            read_all(conn)?;
        }
        _ => return Err(anyhow!("未定义的指令")),
    }
    Ok(())
}

fn read_all(conn: &mut SqliteConnection) -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::models::Cate;
    use diesel_demo::schema::cates::dsl::*;
    let all_cates = cates.select(Cate::as_returning()).load(conn)?;
    println!("all cate:\n{:#?}", all_cates);
    Ok(())
}

fn create_cate(
    conn: &mut SqliteConnection,
    icon: String,
    name: String,
    total: i32,
) -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::models::Cate;
    use diesel_demo::schema::cates::dsl::*;
    let cate = diesel::insert_into(cates)
        .values((icon_url.eq(icon), cate_name.eq(name), live_total.eq(total)))
        .returning(Cate::as_returning())
        .get_result(conn)?;
    println!("create a cate: {:?}", cate);
    Ok(())
}

fn delete_cate(conn: &mut SqliteConnection, cid: i32) -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::models::Cate;
    use diesel_demo::schema::cates::dsl::*;
    let cate = diesel::delete(cates)
        .filter(id.eq(cid))
        .returning(Cate::as_returning())
        .get_result(conn)?;
    println!("del a cate: {:?}", cate);
    Ok(())
}
