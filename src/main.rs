use structs::CoronalMassEjectionResponse;

#[macro_use]
extern crate rocket;

use rocket::response::status;
mod structs;
mod toml_reader;

#[get("/")]
fn goodbye() -> String {
    return toml_reader::get_donki_config().donki_config.api_key;
}

#[get("/coronal-mass-ejections")]
fn get_coronal_mass_ejections() -> status::Accepted<CoronalMassEjectionResponse> {
    return status::Accepted(Some(CoronalMassEjectionResponse::new()));
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![goodbye]).mount(
        "/coronal-mass-ejections",
        routes![get_coronal_mass_ejections],
    )
}
