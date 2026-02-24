use std::process::Command;
use std::fs;
use std::io::Read;
use osmpbf::{Element, ElementReader};

pub struct BoundingBox {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lon: f64,
    pub max_lon: f64,
}

impl BoundingBox {
    pub fn from_center(lat: f64, lon: f64, radius_miles: f64) -> Self {
        let lat_deg_per_mile = 1.0 / 69.0;
        let lon_deg_per_mile = 1.0 / (69.0 * lat.to_radians().cos());
        
        let lat_radius = radius_miles * lat_deg_per_mile;
        let lon_radius = radius_miles * lon_deg_per_mile;
        
        BoundingBox {
            min_lat: lat - lat_radius,
            max_lat: lat + lat_radius,
            min_lon: lon - lon_radius,
            max_lon: lon + lon_radius,
        }
    }
    
    pub fn contains(&self, lat: f64, lon: f64) -> bool {
        lat >= self.min_lat && lat <= self.max_lat &&
        lon >= self.min_lon && lon <= self.max_lon
    }
}

pub fn extract_wikidata_from_pbf(pbf_data: &[u8], bbox: &BoundingBox) -> Result<Vec<(String, String, f64, f64)>, anyhow::Error> {
    let reader = ElementReader::new(std::io::Cursor::new(pbf_data));
    let mut results = Vec::new();
    
    for element in reader.par_map_reduce(
        |element| {
            let mut local = Vec::new();
            match element {
                Element::Node(node) => {
                    if bbox.contains(node.lat(), node.lon()) {
                        let tags: std::collections::HashMap<_, _> = node.tags().collect();
                        if let Some(wikidata) = tags.get("wikidata") {
                            let name = tags.get("name").map(|s| s.to_string()).unwrap_or_default();
                            local.push((name, wikidata.to_string(), node.lat(), node.lon()));
                        }
                    }
                }
                _ => {}
            }
            local
        },
        || Vec::new(),
        |mut a, b| { a.extend(b); a }
    ) {
        results.extend(element);
    }
    
    Ok(results)
}
