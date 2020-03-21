#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rs_stock_portfolio;
use self::models::GlobalQuote;
use self::rs_stock_portfolio::*;
use rocket_contrib::json::Json;

#[get("/")]
fn index() -> Json<GlobalQuote> {
  Json(req().unwrap())
}
adfas
// Can only do 5 requests per minute
// So I will have to automate this on a schedule to grab and store the data.
// Once every night I suppose.
// Unless I can batch these requests somehow.
// It would take several minutes to gather.
// Or I use multiple API keys.
// But working with the db in Rust would be a good task
// The user input for now will be a console app. Will need Clap to do it well.
// Fix error with RSL
// Eventually add TUI, AWS LAMBDA
// Good reference https://github.com/paholg/listentothis-playlist-updater
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
