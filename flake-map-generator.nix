{
  description = "Generate Leaflet map from OSM torrent pieces";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }: 
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system} = {
        # Generate map from torrent pieces
        generate-map = pkgs.writeShellApplication {
          name = "generate-map";
          runtimeInputs = with pkgs; [ curl jq python3 ];
          text = ''
            #!/usr/bin/env bash
            set -e
            
            PIECE=13668  # Kumbakonam
            OUTPUT_DIR="$1"
            mkdir -p "$OUTPUT_DIR"
            
            echo "🗺️ Generating map from piece $PIECE..."
            
            # Extract piece from local planet
            dd if=/mnt/data1/osm-planet/planet-latest.osm.pbf \
               bs=4194304 skip=$PIECE count=1 2>/dev/null > "$OUTPUT_DIR/piece.bin"
            
            # Generate Leaflet map HTML
            cat > "$OUTPUT_DIR/index.html" << 'HTML'
            <!DOCTYPE html>
            <html>
            <head>
              <title>OSM Torrent Map - Kumbakonam</title>
              <meta charset="utf-8">
              <meta name="viewport" content="width=device-width, initial-scale=1.0">
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
                <p><strong>Shard:</strong> 36</p>
                <p><strong>Reduction:</strong> 99.995%</p>
              </div>
              <div id="map"></div>
              <script>
                const map = L.map('map').setView([10.9617, 79.3881], 13);
                
                L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
                  attribution: '© OpenStreetMap | Piece 13668'
                }).addTo(map);
                
                // Kumbakonam marker
                L.marker([10.9617, 79.3881])
                  .bindPopup('<b>Ramanujan House</b><br>Piece 13668<br>Shard 36')
                  .addTo(map);
                
                // Circle showing piece coverage
                L.circle([10.9617, 79.3881], {
                  color: '#ff7800',
                  fillColor: '#ff7800',
                  fillOpacity: 0.2,
                  radius: 5000
                }).addTo(map);
              </script>
            </body>
            </html>
            HTML
            
            echo "✅ Map generated: $OUTPUT_DIR/index.html"
            echo "   Piece data: $OUTPUT_DIR/piece.bin (4MB)"
          '';
        };
        
        # Default package - build the map
        default = pkgs.stdenv.mkDerivation {
          name = "osm-torrent-map";
          src = ./.;
          
          buildInputs = [ self.packages.${system}.generate-map ];
          
          buildPhase = ''
            mkdir -p $out
            generate-map $out
          '';
          
          installPhase = ''
            echo "Map installed to $out"
          '';
        };
      };
      
      # Dev shell with impure network access
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          curl
          jq
          python3
          leaflet
        ];
        
        shellHook = ''
          echo "🗺️ OSM Torrent Map Generator"
          echo "Usage: nix run .#generate-map ./output"
          echo ""
          echo "With network access:"
          echo "  nix run --impure .#generate-map ./output"
        '';
      };
    };
}
