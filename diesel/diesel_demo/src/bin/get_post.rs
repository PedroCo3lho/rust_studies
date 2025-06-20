
use self::models::Post;
use diesel::prelude::*;
use diesel_demo::*;
use std::env::args;

fn main(){
    use self::schema::posts::dsl::posts;

    let id =  args()
        .nth(1)
        .expect("get_posts requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let post =  posts
        .find(id)
        .select(Post::as_select())
        .first(connection)
        .optional();

    match post {
        Ok(Some(post)) => println!("Post with id: {} has a title: {}", post.id, post.title),
        Ok(None) => println!("Unable to find post: {id}"),
        Err(_) => println!("An error occured while fetching post {id}"),
    }
}