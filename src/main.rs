#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::serve::StaticFiles;
use rocket::routes;
use hello_rocket::connection::DbConn;

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", StaticFiles::from("static/"))
        .mount("/api", routes![
               hello_rocket::route_handlers::get_titles,
               hello_rocket::route_handlers::get_post,
               hello_rocket::route_handlers::create_post,
               hello_rocket::route_handlers::delete_post
        ])
        .launch();
}
