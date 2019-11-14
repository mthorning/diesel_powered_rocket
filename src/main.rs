#![feature(proc_macro_hygiene, decl_macro)]

//#[macro_use] extern crate rocket;
//#[macro_use] extern crate rocket_contrib;
//#[macro_use] extern crate serde_derive;

use serde_derive::{Serialize, Deserialize};
use rocket::{routes, get, post};
use rocket_contrib::json;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::{Json, JsonValue};
use hello_rocket::db::handlers;
use rocket::http::{ContentType, Status};
use rocket::response;
use rocket::response::{Responder, Response};
use rocket::request::Request;

#[derive(Debug)]
struct ApiResponse {
    data: JsonValue,
    status: Status
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.data.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[get("/titles")]
fn get_titles() -> ApiResponse {
    match handlers::get_titles() {
        Ok(titles) => ApiResponse {
            data: json!({ "titles": titles }),
            status: Status::Ok,
        },
        Err(error) => ApiResponse {
            data: json!({ "error": format!("DB Error: {:?}", error)}),
            status: Status::NotFound,
        }
    }

}

#[get("/post/<id>")]
fn get_post(id: String) -> ApiResponse {
    match handlers::get_post(id.parse::<i32>().unwrap()) {
        Ok(post) => ApiResponse {
            data: json!({ "post": post }),
            status: Status::Ok,
        },
        Err(error) => ApiResponse {
            data: json!({ "error": format!("DB Error: {:?}", error)}),
            status: Status::NotFound,
        }
    }
}
    
#[derive(Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[post("/new", data = "<new_post>")]
fn create_post(new_post: Json<NewPost>) -> JsonValue {
    let db_response = handlers::create_post(&new_post.title, &new_post.body);
    json!({ "title": db_response.title, "body": db_response.body })
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("static/"))
        .mount("/api", routes![get_titles, get_post, create_post])
        .launch();
}
