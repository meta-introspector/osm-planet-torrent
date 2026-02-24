#!/usr/bin/env bash
# Use browsr CLI to render and check visualizations

set -e

echo "🌐 BROWSR CLI - Render and Check Visualizations"
echo ""

# Check if visualizations exist
if [ ! -f osm_fall_visualization.html ]; then
    echo "⚠️  No visualizations found. Run ./visualize-perf.sh first"
    exit 1
fi

# Use Python validator (browsr-compatible)
echo "🔍 Validating HTML..."
python3 browsr-validator.py osm_fall_visualization.html

echo ""
echo "📊 Validating conformal SVG..."
if [ -f osm_fall_conformal.svg ]; then
    python3 browsr-validator.py osm_fall_conformal.svg
else
    echo "⚠️  osm_fall_conformal.svg not found"
fi

echo ""
echo "📋 Validating register state..."
if [ -f osm_fall_registers.svg ]; then
    python3 browsr-validator.py osm_fall_registers.svg
else
    echo "⚠️  osm_fall_registers.svg not found"
fi

echo ""
echo "⚡ Validating instruction flow..."
if [ -f osm_fall_instructions.svg ]; then
    python3 browsr-validator.py osm_fall_instructions.svg
else
    echo "⚠️  osm_fall_instructions.svg not found"
fi

echo ""
echo "✅ All validations complete!"
echo ""
echo "To view in browser:"
echo "  firefox osm_fall_visualization.html"
echo "  # or"
echo "  python3 -m http.server 8000"
