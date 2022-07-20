use std::fs;
use std::process::exit;
use toml;
use super::structs::donki_config;

pub fn get_donki_config() -> donki_config::Data {
    let file_name = "src/donki_config.toml";
    let contents = match fs::read_to_string(file_name) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("Couldn't read the DONKI config file: `{}`", file_name);
            eprintln!("{}", err);
            exit(1);
        }
    };
    let data: donki_config::Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(err) => {
            eprintln!("Unable to load data from `{}`", file_name);
            eprintln!("{}", err);
            exit(1);
        }
    };
    return data;
}