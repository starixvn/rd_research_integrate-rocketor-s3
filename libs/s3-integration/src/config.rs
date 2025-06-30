use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub bucket: String,
    pub access_id: String,
    pub secret_key: String,
    pub region: String,
    pub target_folder: String,
}
