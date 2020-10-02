#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod api;
pub mod models;

fn main() {
    rocket::ignite()
        .mount("/", routes![api::shortener::index])
        .launch();
}
