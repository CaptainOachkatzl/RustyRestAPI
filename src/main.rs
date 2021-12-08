#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;

#[get("/")]
async fn index() -> &'static str {
    return "hello world";
}

#[get("/hello/<name>")]
async fn greet(name: &str) -> String {
    return format!("hi, {}!", name);
}

#[rocket::main]
async fn main() {
    let server_status = rocket::build()
        .mount("/", routes![index, greet])
        .launch()
        .await;

    match server_status {
        Ok(_) => println!("Server shutdown gracefully."),
        Err(_) => println!("Server execution failure.")
    }
}