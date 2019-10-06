#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize)]
struct PostTitle {
    id: usize,
    title: String,
}

impl PostTitle {
    pub fn get_post_titles() -> Vec<PostTitle> {
        vec![
            PostTitle { id: 1, title: String::from("Post one") },
            PostTitle { id: 2, title: String::from("A follow up") }
        ]
    }
}

#[get("/titles", format = "json")]
fn titles() -> JsonValue {
    json!(PostTitle::get_post_titles())
}

#[derive(Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[post("/new", data = "<new_post>")]
fn new(new_post: Json<NewPost>) -> Json<NewPost> {
    new_post
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("static/"))
        .mount("/api", routes![titles, new])
        .launch();
}
