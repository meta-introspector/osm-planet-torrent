#!/usr/bin/env nix-shell
#! nix-shell -i bash -p rustc cargo protobuf zlib aria2 jq

# Test sparse torrent reading with zkperf_dense (Nix version)

set -e

PIECE=${1:-13668}
LIMIT=${2:-10}
TORRENT_URL="https://planet.openstreetmap.org/pbf/planet-latest.osm.pbf.torrent"
OUTPUT_DIR="/tmp/sparse-test-$$"

echo "🎭 Monster OSM Sparse Torrent Test (Nix)"
echo "========================================="
echo "Piece: $PIECE"
echo "Limit: $LIMIT nodes"
echo ""

mkdir -p "$OUTPUT_DIR"
cd "$OUTPUT_DIR"

# Download torrent metadata
echo "📥 Downloading torrent metadata..."
wget -q "$TORRENT_URL" -O planet.torrent || {
    echo "⚠️  Using cached torrent or local file"
}

# Calculate piece parameters
PIECE_SIZE=16777216  # 16MB
OFFSET=$((PIECE * PIECE_SIZE))
SHARD=$(((PIECE * 71) / 86000))

echo ""
echo "📊 Piece Information:"
echo "  Piece: $PIECE"
echo "  Offset: $OFFSET bytes"
echo "  Size: $PIECE_SIZE bytes"
echo "  Shard: $SHARD/71"
echo ""

# Try sparse download with aria2
echo "🌐 Attempting sparse download (piece $PIECE)..."
timeout 60 aria2c \
    --seed-time=0 \
    --max-upload-limit=1K \
    --select-file=1 \
    --file-allocation=none \
    --bt-save-metadata=true \
    --bt-enable-lpd=false \
    --enable-dht=true \
    --bt-max-peers=20 \
    --max-connection-per-server=3 \
    --split=3 \
    --min-split-size=1M \
    --continue=true \
    --max-tries=2 \
    --retry-wait=3 \
    --timeout=30 \
    --dir=. \
    --out=planet-sparse.osm.pbf \
    planet.torrent 2>&1 | head -20 || echo "⚠️  Partial download (expected)"

# Check if we got any data
if [ -f planet-sparse.osm.pbf ]; then
    SIZE=$(stat -f%z planet-sparse.osm.pbf 2>/dev/null || stat -c%s planet-sparse.osm.pbf)
    echo "✅ Downloaded: $SIZE bytes"
else
    echo "⚠️  No data downloaded, using local planet file if available"
    
    # Fallback to local file
    if [ -f /mnt/data1/osm-planet/planet-latest.osm.pbf ]; then
        echo "📂 Using local planet file"
        ln -s /mnt/data1/osm-planet/planet-latest.osm.pbf planet-sparse.osm.pbf
    else
        echo "❌ No OSM data available"
        exit 1
    fi
fi

echo ""
echo "🔧 Building zkperf_dense with Nix..."
cd ~/projects/osm-planet-torrent
cargo build --release --bin zkperf_dense 2>&1 | grep -E "(Compiling|Finished)" || true

echo ""
echo "⚡ Extracting with zkperf_dense..."
./target/release/zkperf_dense \
    --input "$OUTPUT_DIR/planet-sparse.osm.pbf" \
    --output "$OUTPUT_DIR/nodes.geojson" \
    --piece $PIECE \
    --limit $LIMIT

# Show results
echo ""
echo "📊 Results:"
echo "=========="

if [ -f "$OUTPUT_DIR/nodes.geojson" ]; then
    NODE_COUNT=$(jq '.features | length' "$OUTPUT_DIR/nodes.geojson")
    echo "✅ Extracted: $NODE_COUNT nodes"
    echo ""
    echo "Sample node:"
    jq '.features[0]' "$OUTPUT_DIR/nodes.geojson"
else
    echo "❌ No nodes extracted"
fi

if [ -f "$OUTPUT_DIR/nodes.geojson.witness.json" ]; then
    echo ""
    echo "🔐 ZK Witness:"
    cat "$OUTPUT_DIR/nodes.geojson.witness.json"
fi

echo ""
echo "📁 Output directory: $OUTPUT_DIR"
echo "🎭 Test complete!"
