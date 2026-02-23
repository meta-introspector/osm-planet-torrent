#!/bin/bash
# Test deployed binaries on nginx server

NGINX_ROOT=${NGINX_ROOT:-/var/www/html/osm-monster}

echo "🎭 Monster OSM Quest - Deployment Test"
echo "======================================="
echo "Location: $NGINX_ROOT"
echo ""

if [ ! -d "$NGINX_ROOT" ]; then
    echo "❌ Deployment not found at $NGINX_ROOT"
    exit 1
fi

echo "📋 Version Info:"
cat "$NGINX_ROOT/VERSION"
echo ""

echo "🔧 Testing binaries..."
echo ""

# Test zkperf_dense
echo "1. zkperf_dense"
if "$NGINX_ROOT/bin/zkperf_dense" --help &>/dev/null; then
    echo "   ✅ Working"
else
    echo "   ❌ Failed"
fi

# Test fractran_osm
echo "2. fractran_osm"
if "$NGINX_ROOT/bin/fractran_osm" --help &>/dev/null; then
    echo "   ✅ Working"
else
    echo "   ❌ Failed"
fi

# Test ramanujan_24_walkers
echo "3. ramanujan_24_walkers"
if [ -x "$NGINX_ROOT/bin/ramanujan_24_walkers" ]; then
    echo "   ✅ Executable"
else
    echo "   ❌ Not executable"
fi

# Test walkers_with_lmfdb
echo "4. walkers_with_lmfdb"
if [ -x "$NGINX_ROOT/bin/walkers_with_lmfdb" ]; then
    echo "   ✅ Executable"
else
    echo "   ❌ Not executable"
fi

# Test math_nodes_world
echo "5. math_nodes_world"
if [ -x "$NGINX_ROOT/bin/math_nodes_world" ]; then
    echo "   ✅ Executable"
else
    echo "   ❌ Not executable"
fi

echo ""
echo "📊 Test Results:"
if [ -d "$NGINX_ROOT/results" ]; then
    echo "   Found $(find "$NGINX_ROOT/results" -type f | wc -l) result files"
    ls -lh "$NGINX_ROOT/results" | head -10
else
    echo "   ⚠️  No results directory"
fi

echo ""
echo "🌐 Web Interface:"
if [ -f "$NGINX_ROOT/test-results.html" ]; then
    echo "   ✅ http://localhost/osm-monster/test-results.html"
else
    echo "   ⚠️  test-results.html not found"
fi

if [ -f "$NGINX_ROOT/index.html" ]; then
    echo "   ✅ http://localhost/osm-monster/"
else
    echo "   ⚠️  index.html not found"
fi

echo ""
echo "🧪 Quick functional test..."
OUTPUT="/tmp/test-deployed-$$"
if "$NGINX_ROOT/bin/fractran_osm" --piece 13668 > "$OUTPUT" 2>&1; then
    echo "   ✅ FRACTRAN encoding works"
    cat "$OUTPUT"
else
    echo "   ⚠️  FRACTRAN test failed"
fi

echo ""
echo "🎭 Deployment test complete!"
