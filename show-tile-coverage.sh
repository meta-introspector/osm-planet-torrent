#!/usr/bin/env bash
# Show what tiles we actually have

TILE_DIR="/mnt/data1/osm-planet-torrent/tiles"

echo "📊 Tile Coverage Analysis"
echo ""
echo "Total tiles: $(ls $TILE_DIR | wc -l)"
echo "Total size: $(du -sh $TILE_DIR | awk '{print $1}')"
echo ""

# Pick a tile and show its structure
SAMPLE=$(ls $TILE_DIR | head -1)
echo "=== Sample Tile: $SAMPLE ==="
ls -lh "$TILE_DIR/$SAMPLE/" | head -10
echo ""

# Show first node
echo "Sample node data:"
head -1 "$TILE_DIR/$SAMPLE/nodes_"*.csv | head -3
echo ""

# Cost analysis
echo "=== Cost/Benefit ==="
TILE_SIZE=$(du -sh "$TILE_DIR/$SAMPLE" | awk '{print $1}')
echo "One tile: $TILE_SIZE"
echo "Full dataset: 506M (1889 tiles)"
echo "Full planet: 86GB"
echo ""
echo "Reduction: 506M vs 86GB = 99.4% savings"
echo ""

# What we can do
echo "=== What We Have ==="
echo "✅ 1889 pre-processed tiles"
echo "✅ Direct CSV access (node_id, lat, lon)"
echo "✅ Organized by admin_level"
echo "✅ 506M total (vs 86GB planet)"
echo ""
echo "💰 Cost per query:"
echo "   - Lookup tile: 0 bytes (index)"
echo "   - Read tile: ~268KB average"
echo "   - Parse CSV: instant"
echo ""
echo "🎯 Use case:"
echo "   1. Calculate tile from lat/lon"
echo "   2. Check if tile exists in dataset"
echo "   3. If yes: read CSV directly"
echo "   4. If no: fall back to Overpass API"
