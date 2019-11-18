use serde_derive::Serialize;
use super::schema::posts;

//#[derive(Insertable)]
//#[table_name="posts"]
//pub struct NewPost<'a> {
//    pub title: &'a str,
//    pub body: &'a str,
//}

#[derive(Serialize, Queryable, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: std::time::SystemTime,
}

#[derive(Queryable, Serialize)]
pub struct Title {
    id: i32,
    title: String,
}

