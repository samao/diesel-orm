use std::env::args;

use self::models::Post;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
    use self::schema::posts::dsl::posts;

    let post_id = args()
        .nth(1)
        .expect("get_post requires a posst id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();
    let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional();
    match post {
        Ok(Some(post)) => println!(
            "Post with id: {} has a title: <{}>, published: {}",
            post.id, post.title, post.published
        ),
        Ok(None) => println!("Unable to find post {}", post_id),
        _ => println!("An error occured while fetching post {}", post_id),
    }
}
