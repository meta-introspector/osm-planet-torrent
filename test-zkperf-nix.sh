#!/usr/bin/env nix-shell
#! nix-shell -i bash -p rustc cargo protobuf zlib jq

# Quick test of zkperf_dense with local planet file

PIECE=${1:-13668}
LIMIT=${2:-10}
PLANET="/mnt/data1/osm-planet/planet-latest.osm.pbf"
OUTPUT="/tmp/zkperf-test-$$"

echo "🎭 Monster OSM ZKPerf Quick Test (Nix)"
echo "======================================"
echo "Piece: $PIECE"
echo "Limit: $LIMIT nodes"
echo "Shard: $(((PIECE * 71) / 86000))/71"
echo ""

if [ ! -f "$PLANET" ]; then
    echo "❌ Planet file not found: $PLANET"
    exit 1
fi

mkdir -p "$OUTPUT"

echo "🔧 Building zkperf_dense with Nix..."
cd ~/projects/osm-planet-torrent
cargo build --release --bin zkperf_dense 2>&1 | grep -E "(Compiling|Finished)" || true

echo ""
echo "⚡ Extracting nodes..."
time ./target/release/zkperf_dense \
    --input "$PLANET" \
    --output "$OUTPUT/nodes.geojson" \
    --piece $PIECE \
    --limit $LIMIT

echo ""
echo "📊 Results:"
if [ -f "$OUTPUT/nodes.geojson" ]; then
    NODE_COUNT=$(jq '.features | length' "$OUTPUT/nodes.geojson" 2>/dev/null || echo "?")
    echo "✅ Extracted: $NODE_COUNT nodes"
    echo ""
    jq '.features[0] | {id: .properties.id, lat: .geometry.coordinates[1], lon: .geometry.coordinates[0], tags: .properties.tags}' "$OUTPUT/nodes.geojson" 2>/dev/null || cat "$OUTPUT/nodes.geojson" | head -20
fi

if [ -f "$OUTPUT/nodes.geojson.witness.json" ]; then
    echo ""
    echo "🔐 ZK Witness:"
    cat "$OUTPUT/nodes.geojson.witness.json"
fi

echo ""
echo "📁 Output: $OUTPUT"
echo "🎭 Test complete!"
