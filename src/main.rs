#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use structs::CoronalMassEjectionResponse;

mod structs;
mod toml_reader;

#[get("/")]
fn goodbye() -> String {
    return toml_reader::get_donki_config().donki_config.api_key;
}

#[get("/coronal-mass-ejections")]
fn get_coronal_mass_ejections() -> Json<CoronalMassEjectionResponse> {
    Json(CoronalMassEjectionResponse::new())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![goodbye, get_coronal_mass_ejections])
}
