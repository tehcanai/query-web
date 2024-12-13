#[macro_use]
extern crate rocket;
use csv::Reader;
use rocket::fs::TempFile;
use std::fs::File;

#[get("/")]
fn index() -> &'static str {
    "The site is live"
}

#[post("/upload", data = "<file>")]
async fn upload_csv(mut file: TempFile<'_>) -> &'static str {
    let filepath = "./tmp/uploaded.csv";
    file.persist_to(filepath).await.unwrap();

    // Read the CSV file
    let mut rdr = Reader::from_path(filepath).unwrap();
    for result in rdr.records() {
        let record = result.unwrap();
        println!("{:?}", record);
    }

    "CSV Upload Successful"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, upload_csv])
}
