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
    C {
        #[arg(short = 'c', long)]
        icon: Option<String>,
        #[arg(short, long)]
        big_icon: Option<String>,
        #[arg(short, long, required = true)]
        name: String,
        #[arg(short, long)]
        total: Option<i32>,
    },
    U {
        #[arg(short, long)]
        id: i32,
        #[arg(short = 'c', long)]
        icon: Option<String>,
        #[arg(short, long)]
        big_icon: Option<String>,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        total: Option<i32>,
    },
    D {
        #[arg(short, long)]
        id: i32,
    },
    R,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use diesel_demo::establish_connection;
    let conn = &mut establish_connection();
    let args = Args::parse();
    match args.cmd {
        Some(Command::C {
            icon,
            name,
            total,
            big_icon,
        }) => {
            create_cate(conn, icon, big_icon, name, total)?;
        }
        Some(Command::U {
            id,
            icon,
            big_icon,
            name,
            total,
        }) => {
            update(conn, id, icon, big_icon, name, total)?;
        }
        Some(Command::D { id }) => {
            delete_cate(conn, id)?;
        }
        Some(Command::R) => {
            read_all(conn)?;
        }
        _ => return Err(anyhow!("未定义的指令")),
    }
    Ok(())
}

fn update(
    conn: &mut SqliteConnection,
    input_id: i32,
    icon: Option<String>,
    big_icon: Option<String>,
    name: Option<String>,
    total: Option<i32>,
) -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::models::Cate;
    use diesel_demo::schema::cates;
    use diesel_demo::schema::cates::dsl::*;

    #[derive(Debug, AsChangeset)]
    #[diesel(table_name = cates)]
    struct UpdateCate {
        icon_url: Option<String>,
        img_url: Option<String>,
        cate_name: Option<String>,
        live_total: Option<i32>,
    }

    let cate = diesel::update(cates.filter(id.eq(input_id)))
        .set((&UpdateCate {
            icon_url: icon,
            img_url: big_icon,
            cate_name: name,
            live_total: total,
        },))
        .returning(Cate::as_returning())
        .get_result(conn)?;
    println!("update a cate: {:?}", cate);
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
    icon: Option<String>,
    big_icon: Option<String>,
    name: String,
    total: Option<i32>,
) -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::models::Cate;
    use diesel_demo::schema::cates;
    use diesel_demo::schema::cates::dsl::*;

    #[derive(Debug, Insertable)]
    #[diesel(table_name = cates)]
    struct InsertCate {
        icon_url: Option<String>,
        img_url: Option<String>,
        cate_name: String,
        live_total: Option<i32>,
    }

    let cate = diesel::insert_into(cates)
        .values(&InsertCate {
            cate_name: name,
            icon_url: icon,
            img_url: big_icon,
            live_total: total,
        })
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
