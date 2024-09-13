/* #[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "My First Rocket Project with Rust!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
} */

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Esto es una prueba"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hello])
}