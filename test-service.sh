#!/bin/bash
# Test the tile-shard binary before deploying as service

echo "🧪 Testing tile-shard binary..."

# Check if binary exists
if [ ! -f target/release/tile-shard ]; then
    echo "❌ Binary not found. Building..."
    nix develop /home/mdupont/nix/vendor/rust/cargo2nix -c cargo build --release --bin tile-shard
fi

# Check planet file
if [ ! -f /mnt/data1/osm-planet/planet-latest.osm.pbf ]; then
    echo "❌ Planet file not found at /mnt/data1/osm-planet/planet-latest.osm.pbf"
    exit 1
fi

echo "✅ Planet file: $(du -h /mnt/data1/osm-planet/planet-latest.osm.pbf | cut -f1)"

# Check output directories
mkdir -p /mnt/data1/osm-planet-torrent/tiles
mkdir -p /mnt/data1/osm-planet-torrent/admin

# Test run for 30 seconds
echo "🚀 Running tile-shard for 30 seconds..."
cd /mnt/data1/osm-planet-torrent
timeout 30 /home/mdupont/projects/osm-planet-torrent/target/release/tile-shard || true

# Check output
TILE_COUNT=$(find tiles/ -name "*.csv" 2>/dev/null | wc -l)
TILE_SIZE=$(du -sh tiles/ 2>/dev/null | cut -f1)

echo ""
echo "📊 Results:"
echo "  Tiles created: $TILE_COUNT"
echo "  Total size: $TILE_SIZE"
echo ""

if [ $TILE_COUNT -gt 0 ]; then
    echo "✅ Test successful! Ready to deploy as service."
    exit 0
else
    echo "❌ No tiles created. Check logs."
    exit 1
fi
