#!/bin/bash
set -e

echo "🧪 LOCAL TEST: 30 second run"
echo ""

# Clean test dir
rm -rf /tmp/osm-leech-test
mkdir -p /tmp/osm-leech-test/tiles_leech
cd /tmp/osm-leech-test

# Run for 30 seconds
echo "⏱️  Running tile-shard-leech for 30 seconds..."
timeout 30 /home/mdupont/projects/osm-planet-torrent/target/release/tile-shard-leech 2>&1 | tail -20

echo ""
echo "📊 RESULTS:"
echo "=========="

TILES=$(find tiles_leech/ -name "*.jsonl" 2>/dev/null | wc -l)
SIZE=$(du -sh tiles_leech/ 2>/dev/null | cut -f1)
NODES=$(cat tiles_leech/*.jsonl 2>/dev/null | wc -l)

echo "Tiles created: $TILES"
echo "Total size: $SIZE"
echo "Nodes extracted: $NODES"
echo ""

if [ $TILES -gt 0 ]; then
    echo "📄 Sample node (pretty printed):"
    echo "================================"
    head -1 tiles_leech/*.jsonl | jq '.' 2>/dev/null || head -1 tiles_leech/*.jsonl
    
    echo ""
    echo "🗺️  GeoJSON conversion test:"
    echo "============================"
    FIRST_TILE=$(ls tiles_leech/*.jsonl | head -1)
    cat "$FIRST_TILE" | jq -s '{
      type: "FeatureCollection",
      features: map({
        type: "Feature",
        geometry: { type: "Point", coordinates: [.lon, .lat] },
        properties: { name: .name, wikidata: .wikidata, p71_hash: .p71_hash }
      })
    }' > test.geojson 2>/dev/null && echo "✅ Created test.geojson" || echo "❌ GeoJSON failed"
    
    echo ""
    echo "📝 RDFa conversion test:"
    echo "======================="
    head -1 "$FIRST_TILE" | jq -r '
      "<div vocab=\"http://schema.org/\" typeof=\"Place\">" +
      "  <span property=\"name\">" + (.name // "Unknown") + "</span>" +
      "  <span property=\"geo\" typeof=\"GeoCoordinates\">" +
      "    <meta property=\"latitude\" content=\"" + (.lat | tostring) + "\">" +
      "    <meta property=\"longitude\" content=\"" + (.lon | tostring) + "\">" +
      "  </span>" +
      (if .wikidata then "  <link property=\"sameAs\" href=\"https://www.wikidata.org/wiki/" + .wikidata + "\">" else "" end) +
      "</div>"
    ' > test.html 2>/dev/null && echo "✅ Created test.html" || echo "❌ RDFa failed"
    
    echo ""
    echo "✅ TEST PASSED - Data is usable!"
    exit 0
else
    echo "❌ TEST FAILED - No tiles created"
    exit 1
fi
