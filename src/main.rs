#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Serialize)]
struct GlobalQuote {
  #[serde(rename = "Global Quote")]
  global_quote: HashMap<String, String>,
}

#[get("/")]
fn index() -> Json<GlobalQuote> {
  Json(req().unwrap())
}

#[tokio::main]
async fn req() -> Result<GlobalQuote, Box<dyn std::error::Error>> {
  let resp =
    reqwest::get("https://www.alphavantage.co/query?function=GLOBAL_QUOTE&symbol=MSFT&apikey=demo")
      .await
      .unwrap();
  let json = resp.json::<GlobalQuote>().await.unwrap();
  println!("{:#?} the response", json);
  Ok(json)
}

fn main() {
  rocket::ignite().mount("/", routes![index]).launch();
}
