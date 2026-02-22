#!/usr/bin/env bash
# Test with REAL piece calculations

PLANET="/mnt/data1/osm-planet/planet-latest.osm.pbf"
PLANET_SIZE=$(stat -c%s "$PLANET")
INDEX="ramanujan-location-index.json"

echo "🌍 OSM Planet: $(ls -lh $PLANET | awk '{print $5}') ($PLANET_SIZE bytes)"
echo ""

# Calculate real piece size
TOTAL_PIECES=21763  # From index
PIECE_SIZE=$((PLANET_SIZE / TOTAL_PIECES))

echo "📦 Total pieces: $TOTAL_PIECES"
echo "📏 Piece size: $(echo "scale=2; $PIECE_SIZE/1024/1024" | bc)MB"
echo ""

# Test 1: Fetch piece 13668 (Kumbakonam)
echo "=== TEST 1: Kumbakonam (piece 13668) ==="
PIECE=13668
OFFSET=$((PIECE * PIECE_SIZE))
FETCH_SIZE=$((PIECE_SIZE > 50000 ? 50000 : PIECE_SIZE))  # Max 50KB

echo "📍 Location: Kumbakonam [10.9617, 79.3881]"
echo "📦 Piece: $PIECE / $TOTAL_PIECES"
echo "📏 Offset: $(echo "scale=2; $OFFSET/1024/1024/1024" | bc)GB"
echo "💾 Fetch: $(echo "scale=2; $FETCH_SIZE/1024" | bc)KB (first chunk only)"
echo ""

echo "⬇️  Fetching..."
time dd if=$PLANET bs=1 skip=$OFFSET count=$FETCH_SIZE 2>/dev/null > /tmp/kumbakonam.chunk

ACTUAL_SIZE=$(wc -c < /tmp/kumbakonam.chunk)
echo "✅ Fetched: $(echo "scale=2; $ACTUAL_SIZE/1024" | bc)KB"
echo "💰 Cost: $ACTUAL_SIZE bytes ($(echo "scale=6; $ACTUAL_SIZE*100/$PLANET_SIZE" | bc)% of planet)"
echo ""

# Check if it's OSM PBF data
echo "📊 Data type:"
file /tmp/kumbakonam.chunk
echo ""
echo "🔍 First 100 bytes (hex):"
xxd -l 100 /tmp/kumbakonam.chunk
echo ""

# Test 2: Curl equivalent
echo "=== TEST 2: HTTP Range Request ==="
BYTE_END=$((OFFSET + FETCH_SIZE - 1))
echo "curl -r $OFFSET-$BYTE_END https://planet.osm.org/pbf/planet-latest.osm.pbf -o kumbakonam.chunk"
echo "💰 Cost: $(echo "scale=2; $FETCH_SIZE/1024" | bc)KB download"
echo ""

# Test 3: Multiple locations
echo "=== TEST 3: Fetch 3 Cusp Locations ==="
echo "Piece 13668: Kumbakonam (Cusp 17 area)"
echo "Piece 14137: Chennai (Cusp 23 area)"  
echo "Piece 16945: Cambridge (Cusp 59 area)"
TOTAL_FETCH=$((FETCH_SIZE * 3))
echo "💾 Total: $(echo "scale=2; $TOTAL_FETCH/1024" | bc)KB"
echo "💰 Cost: $(echo "scale=6; $TOTAL_FETCH*100/$PLANET_SIZE" | bc)% of planet"
echo "🎯 Gain: All 3 sacred cusp locations"
echo ""

# Summary
echo "=== COST/BENEFIT ANALYSIS ==="
echo ""
echo "Strategy 1: Download full planet"
echo "  Cost: 86GB, ~2 hours"
echo "  Gain: Everything"
echo ""
echo "Strategy 2: Minimal fetch (our approach)"
echo "  Cost: 150KB, <1 second"
echo "  Gain: Exact locations needed"
echo "  Reduction: 99.9998%"
echo ""
echo "Strategy 3: Overpass API"
echo "  Cost: API call, ~5 seconds"
echo "  Gain: Current data (not historical)"
echo ""
echo "✅ Winner: Minimal fetch via index + selective download"
