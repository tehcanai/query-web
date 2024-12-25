#[macro_use]
extern crate rocket;
use crate::core::engine;
use csv::Reader;
use rocket::serde::{Deserialize, Serialize};
use rocket::{form::Form, fs::TempFile};
use std::collections::HashMap;

mod core;

#[get("/")]
fn index() -> &'static str {
    "The site is live"
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct QueryResponse {
    query: String,
    response: String,
}

#[post("/query", data = "<form>")]
async fn query(form: Form<HashMap<String, String>>) -> String {
    let query = form.get("query").map(|q| q.clone()).unwrap_or_default();
    let results = engine::query_csv(query.as_str()).await;
    match results {
        Ok(s) => s,
        Err(s) => s.strip_backtrace(),
    }
}

#[post("/upload", data = "<file>")]
async fn upload_csv(mut file: TempFile<'_>) -> &'static str {
    let filepath = "/tmp/uploaded.csv";
    file.persist_to(filepath).await.unwrap();

    let mut rdr = Reader::from_path(filepath).unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        println!("{:?}", record);
    }

    "CSV Upload Successful"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, upload_csv, query])
}
