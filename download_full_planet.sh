#!/bin/bash
# Download complete OSM planet file via torrent (85 GB)
# This will be MUCH faster than piece-by-piece with good peers

set -e

TORRENT_URL="https://planet.openstreetmap.org/torrent/planet-latest.osm.pbf.torrent"
OUTPUT_DIR="/mnt/data1/osm-planet"
LOG_FILE="planet_download.log"

echo "🌍 Downloading complete OSM planet file" | tee -a "$LOG_FILE"
echo "📦 Size: ~85 GB" | tee -a "$LOG_FILE"
echo "💾 Output: $OUTPUT_DIR" | tee -a "$LOG_FILE"
echo "⏱️  Started: $(date)" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Download using librqbit (our custom torrent client)
cd "$OUTPUT_DIR"

echo "🚀 Starting torrent download..." | tee -a "$LOG_FILE"
nix develop /home/mdupont/nix/vendor/rust/cargo2nix -c \
    cargo run --manifest-path /home/mdupont/projects/osm-planet-torrent/Cargo.toml \
    -- --torrent-url "$TORRENT_URL" \
    2>&1 | tee -a "$LOG_FILE"

echo "" | tee -a "$LOG_FILE"
echo "✅ Download complete!" | tee -a "$LOG_FILE"
echo "⏱️  Finished: $(date)" | tee -a "$LOG_FILE"

# Show file info
ls -lh planet-latest.osm.pbf | tee -a "$LOG_FILE"
