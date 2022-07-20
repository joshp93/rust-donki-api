#[macro_use] extern crate rocket;

#[get("/")]
fn goodbye() -> &'static str {
    "Goodbye, cruel world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![goodbye])
}