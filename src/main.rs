#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;
use rocket::tokio::time::*;

#[get("/")]
async fn index() -> &'static str {
    return "hello world";
}

#[get("/hello/<name>")]
async fn greet(name: &str) -> String {
    return format!("hi, {}!", name);
}

#[get("/delay")]
async fn delay() -> &'static str {
    sleep(Duration::from_secs(3)).await;
    return "well, this took a while...";
}

#[rocket::main]
async fn main() {
    let server_status = rocket::build()
        .mount("/", routes![index, greet, delay])
        .launch()
        .await;

    match server_status {
        Ok(_) => println!("Server shutdown gracefully."),
        Err(_) => println!("Server execution failure.")
    }
}