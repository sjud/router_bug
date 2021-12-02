#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use rocket::fs::NamedFile;
use std::path::{Path};
#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("../server/static/").join("index.html")).await.ok()
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
        .mount("/static", FileServer::from("../server/static"))
}