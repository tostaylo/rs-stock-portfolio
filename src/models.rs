use super::schema::people;
use super::schema::posts;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Queryable, Debug)]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub published: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
  pub title: &'a str,
  pub body: &'a str,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
#[table_name = "people"]
pub struct Person {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
  pub age: i32,
  pub profession: String,
  pub salary: i32,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct GlobalQuote {
  #[serde(rename = "Global Quote")]
  global_quote: HashMap<String, String>,
}
