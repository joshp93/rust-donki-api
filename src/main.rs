#[macro_use] extern crate rocket;

mod structs;
mod toml_reader;

#[get("/")]
fn goodbye() -> String {
    return toml_reader::get_donki_config().donki_config.api_key;
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![goodbye])
}