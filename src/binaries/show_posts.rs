extern crate diesel;
extern crate rs_stock_portfolio;

use self::diesel::prelude::*;
use self::models::*;
use self::rs_stock_portfolio::*;
use establish_connection::establish_connection;

fn main() {
  use rs_stock_portfolio::schema::posts::dsl::*;

  let connection = establish_connection();
  let results = posts
    .filter(published.eq(true))
    .limit(5)
    .load::<Post>(&connection)
    .expect("Error loading posts");

  println!("Displaying {} posts and {:?}", results.len(), results);
  for post in results {
    println!("{}", post.title);
    println!("----------\n");
    println!("{}", post.body);
  }
}
