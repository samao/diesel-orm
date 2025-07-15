use anyhow::anyhow;
use std::env::args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use diesel::prelude::*;
    use diesel_demo::establish_connection;
    use diesel_demo::models::User;
    use diesel_demo::schema::users::dsl::*;

    let mut conn = establish_connection();
    let method = args().nth(1);
    let username = args().nth(2);

    if let Some(method) = method
        && let Some(username) = username
    {
        if method == "add" {
            let user_head = args().nth(3).map(|head| avatar.eq(head));
            let tag = diesel::insert_into(users)
                .values((user_name.eq(username), user_head))
                .returning(User::as_returning())
                .get_result(&mut conn)?;
            println!("insert a user: {:?}", tag);
            return Ok(());
        } else if method == "remove" {
            let tag_id = diesel::delete(users)
                .filter(user_name.eq(username))
                .returning(User::as_returning())
                .get_result(&mut conn)?;
            println!("remove user: {:?}", tag_id);
            return Ok(());
        }
    }

    Err(anyhow!(
        "add user need name, option head url / remove a user need type it's name"
    ))
}
