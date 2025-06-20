use diesel_demo::*;
use std::io::{stdin, Read};

fn main() {
    let conncetion = &mut establish_connection();

    let mut title: String = String::new();
    let mut body: String = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title: &str = title.trim_end();

    println!("\n Let's write {title} (press {EOF} when finished)\n",);
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(conncetion, &title, &body);
    println!("\nSaved draft {title} with id {}", post.id);

}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";