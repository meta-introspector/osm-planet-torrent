use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata: Option<String>,  // Q entity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osm_node: Option<u64>,     // OSM node ID
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLocations {
    pub user: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wikidata_user: Option<String>,  // Wikidata username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub osm_user: Option<String>,       // OSM username
    pub locations: Vec<Location>,
}

pub fn load_user_locations(user: &str) -> Result<UserLocations, Box<dyn std::error::Error>> {
    let path = PathBuf::from(format!("userdir/{}.json", user));
    let data = fs::read_to_string(path)?;
    let locs: UserLocations = serde_json::from_str(&data)?;
    Ok(locs)
}

pub fn save_user_locations(locs: &UserLocations) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("userdir")?;
    let path = PathBuf::from(format!("userdir/{}.json", locs.user));
    let data = serde_json::to_string_pretty(locs)?;
    fs::write(path, data)?;
    Ok(())
}
