#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;

#[get("/")]
async fn index() -> &'static str {
    return "hello world";
}

#[get("/another")]
async fn another_get() -> &'static str {
    return "another get response";
}

#[rocket::main]
async fn main() {
    let server_status = rocket::build()
        .mount("/", routes![index, another_get])
        .launch()
        .await;

    match server_status {
        Ok(_) => println!("Server shutdown gracefully."),
        Err(_) => println!("Server execution failure.")
    }
}