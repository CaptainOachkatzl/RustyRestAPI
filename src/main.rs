#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;

#[get("/")]
fn index() -> &'static str {
    return "hello world";
}

#[get("/another")]
fn another_get() -> &'static str {
    return "another get response";
}

fn main() {
    rocket::ignite().mount("/", routes![index, another_get]).launch();
}