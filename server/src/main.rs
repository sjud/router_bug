#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};
use rocket::http::Status;
use rocket::Request;

#[get("/<_..>", rank = 10)]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("../server/static/").join("index.html")).await.ok()
}

#[get("/static/<file>")]
async fn files(file:PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("../server/static/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index,files])

}