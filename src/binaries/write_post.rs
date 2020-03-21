extern crate diesel;
extern crate rs_stock_portfolio;

use self::models::{NewPost, Post};
use self::rs_stock_portfolio::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::io::{stdin, Read};

fn main() {
  let connection = establish_connection::establish_connection();

  println!("What would you like your title to be?");
  let mut title = String::new();
  stdin().read_line(&mut title).unwrap();
  let title = &title[..(title.len() - 1)]; // Drop the newline character
  println!(
    "\nOk! Let's write {} (Press {} when finished)\n",
    title, EOF
  );
  let mut body = String::new();
  stdin().read_to_string(&mut body).unwrap();

  let post = create_post(&connection, title, &body);
  println!("\nSaved draft {} with id {}", title, post.id);
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
  use schema::posts;

  let new_post = NewPost {
    title: title,
    body: body,
  };

  diesel::insert_into(posts::table)
    .values(&new_post)
    .get_result(conn)
    .expect("Error saving new post")
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
