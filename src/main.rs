use std::path::Path;

use paste_id::PasteID;

use rocket::build;
use rocket::tokio::fs::File;

#[macro_use] extern crate rocket;

mod paste_id;

#[get("/")]
fn index() -> &'static str {
	"USAGE

	POST /

		accepts raw data in the body of the request and responds with a URL of
	a page containing the body's content

	GET /<id>

		retrieves the content for the paste with id `<id>`
	"
}

#[get("/<id>")]
async fn retrieve(id: &str) -> Option<File> {
	let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "uploads");
	let filename = Path::new(upload_dir).join(id);

	File::open(&filename).await.ok()
}

#[launch]
fn rocket() -> _ {
	build().mount("/", routes![index, retrieve])
}
