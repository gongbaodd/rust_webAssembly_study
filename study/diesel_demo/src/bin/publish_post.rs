extern crate diesel_demo;
extern crate diesel;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use std::env::args;

fn main() {
    use diesel_demo::schema::posts::dsl::{posts, published};

    let id = args().nth(1).expect("Need ID!")
        .parse::<i32>().expect("Invalid ID!");
    
    let connection = establish_connection();

    let _post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .execute(&connection)
        .unwrap();

    let post: models::Post = posts.find(id)
        .first(&connection)
        .unwrap_or_else(|_| {
            panic!("Unable to find post {}", id)
        });

    println!("Published post {}", post.id);
}