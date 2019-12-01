#![feature(proc_macro_hygiene, decl_macro)]

extern crate diesel;

pub mod db;
pub mod route_handlers;

pub mod connection {

    use rocket_contrib::databases::diesel;
    use rocket_contrib::database;

    #[database("my_db")]
    pub struct DbConn(diesel::PgConnection);
}

