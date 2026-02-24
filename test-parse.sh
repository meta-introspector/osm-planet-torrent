#!/bin/bash
# Test JSON parsing

echo "Testing first line..."
zcat /mnt/data1/osm-planet/leech-tiles/tiles_leech/tile_14_38_00.jsonl.gz | head -1 > /tmp/test.json
cat /tmp/test.json | jq '.' > /dev/null && echo "✅ jq can parse" || echo "❌ jq failed"

echo ""
echo "Testing Rust parse..."
./target/release/bbs-map 2>&1 | head -20
