use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UrlShortener {
    pub hash: String,
    pub url: String,
    pub lifetime: i64,
}
