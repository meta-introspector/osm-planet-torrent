#!/usr/bin/env bash
# Use exact tile positions from /mnt/data1/osm-planet-torrent/

TILE_DIR="/mnt/data1/osm-planet-torrent/tiles"

echo "🗺️ Using Exact Tile Positions"
echo ""
echo "Dataset: $(ls $TILE_DIR | wc -l) tiles"
echo ""

# Example: Find Kumbakonam tile
# Kumbakonam: 10.9617°N, 79.3881°E
LAT=10.9617
LON=79.3881

# Calculate tile indices (same as our sharding)
TILE_LAT=$(echo "scale=0; (($LAT + 90) * 100) % 71" | bc)
TILE_LON=$(echo "scale=0; (($LON + 180) * 100) % 59" | bc)

echo "=== Kumbakonam Lookup ==="
echo "Coordinates: [$LAT, $LON]"
echo "Tile: tile_${TILE_LAT}_${TILE_LON}"
echo ""

# Check if tile exists
TILE_PATH="$TILE_DIR/tile_${TILE_LAT}_${TILE_LON}"
if [ -d "$TILE_PATH" ]; then
    echo "✅ Tile found!"
    echo "Files:"
    ls -lh "$TILE_PATH"
    echo ""
    echo "Sample nodes:"
    head -5 "$TILE_PATH"/nodes_*.csv
    echo ""
    
    # Count total nodes
    TOTAL=$(cat "$TILE_PATH"/nodes_*.csv | wc -l)
    echo "Total nodes in tile: $TOTAL"
    echo ""
    
    # Calculate cost
    SIZE=$(du -sh "$TILE_PATH" | awk '{print $1}')
    echo "💰 Cost: $SIZE (vs 86GB full planet)"
else
    echo "❌ Tile not found"
    echo "Available tiles near this location:"
    ls "$TILE_DIR" | grep -E "tile_${TILE_LAT}_|tile_.*_${TILE_LON}" | head -10
fi
echo ""

# Show what we have
echo "=== Available Tiles Summary ==="
echo "Total tiles: $(ls $TILE_DIR | wc -l)"
echo "Total size: $(du -sh $TILE_DIR | awk '{print $1}')"
echo ""

# Sample 5 random tiles
echo "Sample tiles:"
ls "$TILE_DIR" | shuf | head -5 | while read tile; do
    nodes=$(find "$TILE_DIR/$tile" -name "*.csv" | wc -l)
    size=$(du -sh "$TILE_DIR/$tile" | awk '{print $1}')
    echo "  $tile: $nodes admin levels, $size"
done
echo ""

echo "✅ EXACT POSITIONS AVAILABLE!"
echo "   - No need to decode PBF"
echo "   - No need to fetch from planet"
echo "   - Direct CSV access"
echo "   - Cost: KB per tile vs GB for planet"
