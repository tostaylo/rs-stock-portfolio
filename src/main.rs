#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug, Serialize)]
struct Snapshot {
  #[serde(rename = "Meta Data")]
  meta_data: HashMap<String, String>,

  #[serde(rename = "Time Series (Daily)")]
  time_series_daily: HashMap<String, HashMap<String, String>>,
}

#[get("/")]
fn index() -> Json<Snapshot> {
  Json(req().unwrap())
}

#[tokio::main]
async fn req() -> Result<Snapshot, Box<dyn std::error::Error>> {
  let resp = reqwest::get(
    "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=MSFT&apikey=demo",
  )
  .await
  .unwrap();
  let json = resp.json::<Snapshot>().await.unwrap();
  println!("{:#?} the response", json);
  Ok(json)
}

fn main() {
  rocket::ignite().mount("/", routes![index]).launch();
}
