#![feature(proc_macro_hygiene, decl_macro)]

//#[macro_use] extern crate rocket;
//#[macro_use] extern crate rocket_contrib;
//#[macro_use] extern crate serde_derive;

use serde::{Serialize};
use serde_derive::{Deserialize, Serialize};
use rocket::{routes, get, post, delete};
use rocket_contrib::json;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::{Json, JsonValue};
use hello_rocket::db::handlers;
use rocket::http::{ContentType, Status};
use rocket::response;
use rocket::response::{Responder, Response};
use rocket::request::Request;
use diesel::QueryResult;

#[derive(Debug)]
struct ApiResponse {
    data: JsonValue,
    status: Status
}

impl ApiResponse {
    pub fn from<T: Serialize>(db_response: QueryResult<T>) -> ApiResponse {
        match db_response {
            Ok(response) => ApiResponse {
                data: json!(response),
                status: Status::Ok,
            },
            Err(error) => ApiResponse {
                data: json!({ "error": format!("{:?}", error)}),
                status: Status::NotFound,
            }
        }
    }
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
    ApiResponse::from(handlers::get_titles())
}

#[get("/post/<id>")]
fn get_post(id: String) -> ApiResponse {
    ApiResponse::from(handlers::get_post(id.parse::<i32>().unwrap()))
}
    
#[derive(Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[post("/new", data = "<new_post>")]
fn create_post(new_post: Json<NewPost>) -> ApiResponse {
    ApiResponse::from(handlers::create_post(&new_post.title, &new_post.body))
}

#[delete("/delete/<id>")]
    fn delete_post(id: String) -> ApiResponse {
        ApiResponse::from(handlers::delete_post(id.parse::<i32>().unwrap()))
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("static/"))
        .mount("/api", routes![get_titles, get_post, create_post, delete_post])
        .launch();
}
