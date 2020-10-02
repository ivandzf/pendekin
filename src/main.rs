#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod api;

fn main() {
    rocket::ignite()
        .mount("/", routes![api::short::index])
        .launch();
}
