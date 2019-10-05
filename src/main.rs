#![feature(proc_macro_hygiene, decl_macro)]
use rocket_contrib::serve::StaticFiles;
#[macro_use] extern crate rocket;

#[get("/titles")]
fn titles() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("static/"))
        .mount("/api", routes![titles])
        .launch();
}
