{ pkgs ? import <nixpkgs> {} }:

let
  # Ramanujan's birthplace
  lat = "10.9617";
  lon = "79.3881";
  
in pkgs.stdenv.mkDerivation {
  name = "kumbakonam-osm-data";
  
  src = ./.;
  
  buildInputs = [ pkgs.python3 pkgs.curl pkgs.jq ];
  
  # Allow network access
  outputHashMode = "recursive";
  outputHashAlgo = "sha256";
  outputHash = pkgs.lib.fakeSha256;
  
  buildPhase = ''
    echo "🏛️ Querying OSM data for Kumbakonam (Ramanujan's birthplace)"
    echo "📍 Location: ${lat}°N, ${lon}°E"
    echo ""
    
    # Use local index
    cp ${./complete_spatial_index_v2.jsonl} index.jsonl
    
    # Find matching pieces
    python3 << 'EOF'
import json
import os

lat = float("${lat}")
lon = float("${lon}")

kb_id = 5000000000 % 71
kb_lat = int(((lat + 90) * 100) % 41)
kb_lon = int(((lon + 180) * 100) % 31)

print(f"🎯 Target bucket: ({kb_id}, {kb_lat}, {kb_lon})")

matches = []
with open('index.jsonl') as f:
    for line in f:
        p = json.loads(line)
        if (p['bucket_id'] == kb_id and 
            p['bucket_lat'] == kb_lat and 
            p['bucket_lon'] == kb_lon):
            matches.append(p)

print(f"📦 Found {len(matches)} matching pieces")
print("")

for m in matches:
    print(f"Piece {m['piece_id']}: {m['node_count']:,} nodes, {m['wikidata_count']} wikidata")
    with open('pieces.txt', 'a') as f:
        f.write(f"{m['piece_id']},{m['byte_offset']},{m['node_count']},{m['wikidata_count']}\n")
EOF

    # Download pieces
    if [ -f pieces.txt ]; then
      while IFS=',' read -r piece_id offset nodes wikidata; do
        echo ""
        echo "⬇️  Downloading piece $piece_id (offset $offset, $nodes nodes, $wikidata wikidata)..."
        
        end=$((offset + 4194304 - 1))
        curl -s -r $offset-$end \
          https://planet.openstreetmap.org/pbf/planet-latest.osm.pbf \
          -o piece_$piece_id.pbf
        
        echo "✅ Downloaded $(ls -lh piece_$piece_id.pbf | awk '{print $5}')"
      done < pieces.txt
    fi
  '';
  
  installPhase = ''
    mkdir -p $out
    
    echo "📊 Summary for Kumbakonam area:" > $out/summary.txt
    echo "Location: ${lat}°N, ${lon}°E" >> $out/summary.txt
    echo "" >> $out/summary.txt
    
    if [ -f pieces.txt ]; then
      cat pieces.txt >> $out/summary.txt
      cp piece_*.pbf $out/ 2>/dev/null || true
    fi
    
    echo "" >> $out/summary.txt
    echo "✅ Query complete" >> $out/summary.txt
    
    cat $out/summary.txt
  '';
}
