#!/usr/bin/env python3
import json
import sys

def generate_map(index_file, output_file):
    with open(index_file) as f:
        data = json.load(f)
    
    user = data['user']
    wikidata = data.get('wikidata_user', 'N/A')
    locations = data['locations']
    
    # Generate Leaflet map
    html = f"""<!DOCTYPE html>
<html>
<head>
    <title>{user}'s OSM Torrent Locations</title>
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
        
        var locations = {json.dumps(locations)};
        
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
            div.innerHTML = '<h3>{user}</h3>';
            div.innerHTML += 'Wikidata: <a href="https://www.wikidata.org/wiki/{wikidata}">{wikidata}</a><br>';
            div.innerHTML += 'Locations: ' + locations.length + '<br>';
            div.innerHTML += 'Total pieces: {data["pieces"]}';
            return div;
        }};
        info.addTo(map);
    </script>
</body>
</html>"""
    
    with open(output_file, 'w') as f:
        f.write(html)
    
    print(f"✓ Map generated: {output_file}")

if __name__ == '__main__':
    if len(sys.argv) != 3:
        print("Usage: generate_map.py <index.json> <output.html>")
        sys.exit(1)
    
    generate_map(sys.argv[1], sys.argv[2])
