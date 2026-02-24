#!/bin/bash
# Download specific byte ranges from OSM planet file
# Much faster than torrent with slow peers!

set -e

URL="https://planet.openstreetmap.org/pbf/planet-latest.osm.pbf"
PIECE_SIZE=4194304  # 4 MB

# Function to download a specific piece
download_piece() {
    local piece_id=$1
    local start_byte=$((piece_id * PIECE_SIZE))
    local end_byte=$((start_byte + PIECE_SIZE - 1))
    local output="piece_${piece_id}_direct.pbf"
    
    echo "📦 Downloading piece $piece_id (bytes $start_byte-$end_byte)..."
    
    curl -L -r "$start_byte-$end_byte" \
         -o "$output" \
         --progress-bar \
         "$URL"
    
    if [ -f "$output" ]; then
        local size=$(stat -f%z "$output" 2>/dev/null || stat -c%s "$output")
        echo "   ✓ Downloaded $size bytes"
    fi
}

# Download first block of every 71st piece (Monster Group sampling)
echo "🗺️  Downloading OSM planet pieces via HTTP range requests"
echo "📊 Sampling every 71st piece (307 pieces = ~1.2 GB)"
echo ""

for ((i=0; i<21763; i+=71)); do
    download_piece $i
    
    # Process immediately
    echo "   Processing piece $i..."
    nix develop /home/mdupont/nix/vendor/rust/cargo2nix -c cargo run --bin decode-zlib -- "piece_${i}_direct.pbf" 2>&1 | grep -E "(Decompressed|Error)" || true
    
    # Rate limit
    sleep 1
done

echo ""
echo "✅ Download complete!"
echo "📍 Building spatial index..."
nix develop /home/mdupont/nix/vendor/rust/cargo2nix -c cargo run --bin build-spatial-index
