use serde::{Serialize};
use serde_derive::{Deserialize, Serialize};

use rocket::{get, post, delete};
use rocket::http::{ContentType, Status};
use rocket::response::{Result, Responder, Response};
use rocket::request::Request;

use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;
use crate::db::handlers;
use diesel::QueryResult;

#[derive(Debug)]
pub struct ApiResponse {
    pub data: JsonValue,
    pub status: Status
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
    fn respond_to(self, req: &Request) -> Result<'r> {
        Response::build_from(self.data.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[get("/titles")]
pub fn get_titles(conn: crate::connection::DbConn) -> ApiResponse {
    ApiResponse::from(handlers::get_titles(&*conn))
}

#[get("/post/<id>")]
pub fn get_post(id: String) -> ApiResponse {
    ApiResponse::from(handlers::get_post(id.parse::<i32>().unwrap()))
}
    
#[derive(Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[post("/new", data = "<new_post>")]
pub fn create_post(new_post: Json<NewPost>) -> ApiResponse {
    ApiResponse::from(handlers::create_post(&new_post.title, &new_post.body))
}

#[delete("/delete/<id>")]
    pub fn delete_post(id: String) -> ApiResponse {
        ApiResponse::from(handlers::delete_post(id.parse::<i32>().unwrap()))
}
