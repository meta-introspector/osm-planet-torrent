use std::fs;
use std::path::Path;
use serde_json::Value;

pub fn generate_map(index_file: &str, output_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string(index_file)?;
    let json: Value = serde_json::from_str(&data)?;
    
    let user = json["user"].as_str().unwrap_or("unknown");
    let wikidata = json.get("wikidata_user").and_then(|v| v.as_str()).unwrap_or("N/A");
    let locations = json["locations"].as_array().unwrap_or(&vec![]);
    
    let locations_json = serde_json::to_string(locations)?;
    let pieces = json["pieces"].as_i64().unwrap_or(0);
    
    let html = format!(r#"<!DOCTYPE html>
<html>
<head>
    <title>{}'s OSM Torrent Locations</title>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
    <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
    <style>
        body {{ margin: 0; padding: 0; font-family: sans-serif; }}
        #map {{ height: 100vh; }}
        .info {{ padding: 10px; background: white; border-radius: 5px; }}
        .info h3 {{ margin: 0 0 10px; }}
    </style>
</head>
<body>
    <div id="map"></div>
    <script>
        var map = L.map('map').setView([20, 20], 2);
        
        L.tileLayer('https://{{s}}.tile.openstreetmap.org/{{z}}/{{x}}/{{y}}.png', {{
            attribution: '© OpenStreetMap contributors'
        }}).addTo(map);
        
        var locations = {};
        
        locations.forEach(function(loc) {{
            var marker = L.marker([loc.lat, loc.lon]).addTo(map);
            var popup = '<b>' + loc.name + '</b><br>';
            if (loc.wikidata) {{
                popup += 'Wikidata: <a href="https://www.wikidata.org/wiki/' + loc.wikidata + '">' + loc.wikidata + '</a><br>';
            }}
            popup += 'Piece: ' + loc.piece + '<br>';
            popup += 'Shard: ' + loc.shard + ' (mod 71)';
            marker.bindPopup(popup);
        }});
        
        var info = L.control({{position: 'topright'}});
        info.onAdd = function(map) {{
            var div = L.DomUtil.create('div', 'info');
            div.innerHTML = '<h3>{}</h3>';
            div.innerHTML += 'Wikidata: <a href="https://www.wikidata.org/wiki/{}">{}</a><br>';
            div.innerHTML += 'Locations: ' + locations.length + '<br>';
            div.innerHTML += 'Total pieces: {}';
            return div;
        }};
        info.addTo(map);
    </script>
</body>
</html>"#, user, locations_json, user, wikidata, wikidata, pieces);
    
    fs::create_dir_all(Path::new(output_file).parent().unwrap())?;
    fs::write(output_file, html)?;
    
    println!("✓ Map generated: {}", output_file);
    Ok(())
}
