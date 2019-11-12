use diesel::prelude::*;
use crate::db::schema::posts;
use crate::db::models::{Title, Post, NewPost};
use crate::db::connect;
use diesel::pg::PgConnection;


pub fn create_post<'a>(title: &'a str, body: &'a str) -> Post {
    let conn: PgConnection = connect::establish_connection();
    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&conn)
        .expect("Error saving new post")
}

pub fn get_titles() -> QueryResult<Vec<Title>> {
    use crate::db::schema::posts::dsl::*;
    let conn: PgConnection = connect::establish_connection();
    let result = posts.select((id, title)).load::<Title>(&conn);
    result
}

pub fn get_post(id: i32) -> QueryResult<Post> {
    let conn: PgConnection = connect::establish_connection();
    let result = posts::table.find(id).get_result::<Post>(&conn);
    result
}
