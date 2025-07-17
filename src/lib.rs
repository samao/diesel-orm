use std::env;
use diesel::prelude::*;
use dotenvy::dotenv;
use tracing::{level_filters::LevelFilter, subscriber::set_global_default};
use tracing_subscriber::FmtSubscriber;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let subscriber = FmtSubscriber::builder().with_max_level(LevelFilter::DEBUG).finish();
    set_global_default(subscriber).ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL muse be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}