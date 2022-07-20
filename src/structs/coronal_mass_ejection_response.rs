use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
#[derive(Serialize)]
pub struct CoronalMassEjectionResponse {
    #[serde(alias = "activityID")]
    activity_id: String,
    #[serde(alias = "catalog")]
    catalog: String,
    #[serde(alias = "startTime")]
    start_time: String,
    #[serde(alias = "sourceLocation")]
    source_location: String,
    #[serde(alias = "link")]
    link: String,
    #[serde(alias = "note")]
    note: String,
    #[serde(alias = "instruments")]
    instruments: Vec<Instrument>,
    #[serde(alias = "cmeAnalyses")]
    cme_analyses: Vec<CMEAnalysis>,
}

impl CoronalMassEjectionResponse {
    pub(crate) fn new() -> Self {
        CoronalMassEjectionResponse {
            activity_id: String::from(""),
            catalog: String::from(""),
            start_time: String::from(""),
            source_location: String::from(""),
            link: String::from(""),
            note: String::from(""),
            instruments: Vec::new(),
            cme_analyses: Vec::new(),

        }
    }
}

#[derive(Deserialize)]
#[derive(Serialize)]
pub(crate) struct Instrument {
    #[serde(alias = "displayName")]
    display_name: String,
}

#[derive(Deserialize)]
#[derive(Serialize)]
pub(crate) struct CMEAnalysis {
    #[serde(alias = "time21_5")]
    time21_5: String,
    #[serde(alias = "latitude")]
    latitude: f32,
    #[serde(alias = "longitude")]
    longitude: f32,
    #[serde(alias = "halfAngle")]
    half_angle: f32,
    #[serde(alias = "speed")]
    speed: f32,
    #[serde(alias = "type")]
    cmea_type: String,
    #[serde(alias = "isMostAccurate")]
    is_most_accurate: bool,
    #[serde(alias = "note")]
    note: String,
    #[serde(alias = "levelOfData")]
    level_of_data: i32,
    #[serde(alias = "link")]
    link: String,
    #[serde(alias = "enlilList")]
    enlil_list: Vec<ENLIL>,
}

#[derive(Deserialize)]
#[derive(Serialize)]
pub(crate) struct ENLIL {
    #[serde(alias = "modelCompletionTime")]
    model_completion_time: String,
    #[serde(alias = "au")]
    au: f32,
    #[serde(alias = "isEarthGB")]
    is_earth_gb: bool,
    #[serde(alias = "link")]
    link: String,
    #[serde(alias = "cmeIDs")]
    cme_ids: Vec<String>,
}
