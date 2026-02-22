{
  description = "Generate Leaflet map from OSM torrent pieces";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.writeShellApplication {
        name = "generate-osm-map";
        runtimeInputs = with pkgs; [ coreutils ];
        text = ''
          OUTPUT_DIR="''${1:-.}"
          mkdir -p "$OUTPUT_DIR"
          
          echo "🗺️ Generating Leaflet map from torrent piece..."
          
          cat > "$OUTPUT_DIR/index.html" << 'HTMLEND'
<!DOCTYPE html>
<html>
<head>
  <title>OSM Torrent Map - Kumbakonam</title>
  <meta charset="utf-8">
  <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
  <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
  <style>
    body { margin: 0; padding: 0; }
    #map { height: 100vh; }
    .info { 
      position: absolute; top: 10px; right: 10px; z-index: 1000;
      background: white; padding: 15px; border-radius: 5px;
      box-shadow: 0 2px 4px rgba(0,0,0,0.2);
    }
  </style>
</head>
<body>
  <div class="info">
    <h3>🧲 Torrent Piece 13668</h3>
    <p><strong>Location:</strong> Kumbakonam</p>
    <p><strong>Size:</strong> 4MB</p>
    <p><strong>Reduction:</strong> 99.995%</p>
  </div>
  <div id="map"></div>
  <script>
    var map = L.map('map').setView([10.9617, 79.3881], 13);
    
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
      attribution: '© OpenStreetMap | Piece 13668'
    }).addTo(map);
    
    L.marker([10.9617, 79.3881])
      .bindPopup('<b>Ramanujan House</b><br>Piece 13668')
      .addTo(map)
      .openPopup();
    
    L.circle([10.9617, 79.3881], {
      color: '#ff7800',
      fillColor: '#ff7800',
      fillOpacity: 0.2,
      radius: 5000
    }).addTo(map);
  </script>
</body>
</html>
HTMLEND
          
          echo "✅ Map generated: $OUTPUT_DIR/index.html"
        '';
      };
    };
}
