//! Generate static maps from torrent-downloaded shards

use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = PathBuf::from("data/torrents/geo_shards");
    let output = PathBuf::from("public/maps");
    
    fs::create_dir_all(&output)?;
    
    let mut features = Vec::new();
    
    if input.exists() {
        for entry in fs::read_dir(&input)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let data = fs::read_to_string(&path)?;
                if let Ok(shard) = serde_json::from_str::<serde_json::Value>(&data) {
                    features.push(shard);
                }
            }
        }
    }
    
    let geojson = serde_json::json!({
        "type": "FeatureCollection",
        "features": features
    });
    
    fs::write(output.join("map.geojson"), serde_json::to_string_pretty(&geojson)?)?;
    
    let html = r#"<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>OSM Planet Map - Monster Group</title>
  <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
  <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
  <style>
    body { margin: 0; padding: 0; }
    #map { height: 100vh; width: 100%; }
  </style>
</head>
<body>
  <div id="map"></div>
  <script>
    const map = L.map('map').setView([0, 0], 2);
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png').addTo(map);
    fetch('map.geojson').then(r => r.json()).then(data => L.geoJSON(data).addTo(map));
  </script>
</body>
</html>"#;
    
    fs::write(output.join("index.html"), html)?;
    
    println!("✅ Generated {} features", features.len());
    
    Ok(())
}
