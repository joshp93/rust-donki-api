use serde::Deserialize;

#[derive(Deserialize)]
pub struct Data {
    pub(crate) donki_config: DonkiConfig
}

#[derive(Deserialize)]
pub(crate) struct DonkiConfig {
    pub(crate) api_key: String
}
