#[macro_use]
extern crate rocket;
use crate::lib::engine;
use csv::Reader;
use rocket::fs::TempFile;
use std::{fs::File, ptr::read};

mod lib;

#[get("/")]
fn index() -> &'static str {
    "The site is live"
}

#[get("/process")]
async fn process_csv() -> String {
    let results = engine::query().await;
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
    rocket::build().mount("/", routes![index, upload_csv, process_csv])
}
