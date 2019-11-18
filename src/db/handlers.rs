use diesel::prelude::*;
use diesel::insert_into;
use crate::db::models::{Title, Post};
use crate::db::connect;
use diesel::pg::PgConnection;
use crate::db::schema::posts::dsl::*;

pub fn get_titles() -> QueryResult<Vec<Title>> {
    let conn: PgConnection = connect::establish_connection();
    let result = posts.select((id, title)).order(created_at.desc()).load::<Title>(&conn);
    result
}

pub fn get_post(requested_id: i32) -> QueryResult<Post> {
    let conn: PgConnection = connect::establish_connection();
    let result = posts.find(requested_id).get_result::<Post>(&conn);
    result
}

pub fn create_post<'a>(new_title: &'a str, new_body: &'a str) -> QueryResult<usize> {
    let conn: PgConnection = connect::establish_connection();

    insert_into(posts)
        .values((title.eq(new_title), body.eq(new_body)))
        .execute(&conn)
}

pub fn delete_post(id_for_deletion: i32) -> QueryResult<usize> {
    let conn: PgConnection = connect::establish_connection();

    diesel::delete(posts.filter(id.eq(id_for_deletion))).execute(&conn)
}
