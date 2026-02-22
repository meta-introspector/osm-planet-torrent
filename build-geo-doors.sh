#!/usr/bin/env bash
# Generate geographic doors

declare -A GEO_DOORS=(
  ["0"]="Pacific Ocean:🌊:0,0"
  ["5"]="Himalayas:🏔️:28.0,84.0"
  ["10"]="Amazon:🌳:-3.0,-60.0"
  ["30"]="New York:🏙️:40.7,-74.0"
  ["35"]="Tokyo:🗼:35.7,139.7"
  ["40"]="London:🏰:51.5,-0.1"
  ["70"]="Omega Point:⚡:0,0"
)

for door in "${!GEO_DOORS[@]}"; do
  IFS=: read -r name emoji coords <<< "${GEO_DOORS[$door]}"
  IFS=, read -r lat lon <<< "$coords"
  
  cat > "templates/door-geo-${door}.html" << HTML
<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>Door $door - $name</title>
  <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
  <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
  <style>
    body { margin: 0; }
    #map { height: 100vh; width: 100%; }
    .info { 
      position: absolute; top: 20px; left: 60px; z-index: 1000;
      background: rgba(0,0,0,0.8); padding: 15px; border-radius: 8px;
      color: #00ff00; font-family: monospace;
    }
  </style>
</head>
<body>
  <div class="info">
    <h2>🚪 Door $door - $name</h2>
    <p>$emoji Location: [$lat°, $lon°]</p>
    <p>10-Fold Way: Level $door</p>
    <p><a href="index.html" style="color: #00ff00;">← Back to Doors</a></p>
  </div>
  <div id="map"></div>
  <script>
    const map = L.map('map').setView([$lat, $lon], $door === 0 ? 2 : 10);
    
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '© OpenStreetMap'
    }).addTo(map);
    
    // Marker
    const marker = L.marker([$lat, $lon]).addTo(map);
    marker.bindPopup('<b>$name</b><br>Door $door<br>$emoji').openPopup();
    
    // Circle
    L.circle([$lat, $lon], {
      color: '#00ff00',
      fillColor: '#00ff00',
      fillOpacity: 0.2,
      radius: 50000
    }).addTo(map);
  </script>
</body>
</html>
HTML

  cp "templates/door-geo-${door}.html" "public_html/doors/door-${door}.html"
  echo "✅ Door $door - $name"
done

echo ""
echo "✅ Built ${#GEO_DOORS[@]} geographic doors"
