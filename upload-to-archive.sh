#!/bin/bash
# Upload Monster OSM Quest data to Archive.org

set -e

echo "🎭 Monster OSM Quest - Archive.org Upload"
echo "=========================================="

# Check for ia tool
if ! command -v ia &> /dev/null; then
    echo "Installing internetarchive..."
    pip install internetarchive
fi

# Configure (requires API keys)
echo "📝 Configure with: ia configure"
echo "   Get keys from: https://archive.org/account/s3.php"

# Create collections
COLLECTIONS=(
    "lmfdb-monster-71-shards"
    "ramanujan-osm-locations"
    "monster-musical-periodic-table"
)

# Upload LMFDB data
echo ""
echo "📤 Uploading LMFDB data..."
if [ -f "/mnt/data1/spool/experiments_monster/lmfdb_71_shards.json" ]; then
    ia upload lmfdb-monster-71-shards \
        /mnt/data1/spool/experiments_monster/lmfdb_71_shards.json \
        /mnt/data1/spool/experiments_monster/lmfdb_math_functions.json \
        /mnt/data1/spool/experiments_monster/prolog/lmfdb_knowledge_base.pl \
        --metadata="title:LMFDB Monster 71 Shards" \
        --metadata="description:L-functions and Modular Forms Database mapped to Monster Group 71 shards" \
        --metadata="subject:mathematics;monster-group;lmfdb;number-theory" \
        --metadata="licenseurl:https://creativecommons.org/licenses/by-sa/4.0/" \
        --metadata="creator:Meta-Introspector"
    echo "✅ LMFDB uploaded"
else
    echo "⚠️  LMFDB data not found"
fi

# Upload Ramanujan locations
echo ""
echo "📤 Uploading Ramanujan locations..."
if [ -f "ramanujan-location-index.json" ]; then
    ia upload ramanujan-osm-locations \
        ramanujan-location-index.json \
        /tmp/zkperf_final.geojson \
        --metadata="title:Ramanujan OSM Locations" \
        --metadata="description:8 biographical locations of Srinivasa Ramanujan mapped to OSM pieces" \
        --metadata="subject:ramanujan;mathematics;biography;osm;geography" \
        --metadata="licenseurl:https://creativecommons.org/licenses/by/4.0/" \
        --metadata="creator:Meta-Introspector"
    echo "✅ Ramanujan locations uploaded"
else
    echo "⚠️  Ramanujan data not found"
fi

# Upload Musical Periodic Table
echo ""
echo "📤 Uploading Musical Periodic Table..."
if [ -f "/mnt/data1/spool/experiments_monster/MUSICAL_PERIODIC_TABLE.md" ]; then
    ia upload monster-musical-periodic-table \
        /mnt/data1/spool/experiments_monster/MUSICAL_PERIODIC_TABLE.md \
        --metadata="title:Musical Periodic Table of Monster Group Primes" \
        --metadata="description:15 Monster Group primes mapped to harmonic frequencies with semantic emoji annotations" \
        --metadata="subject:monster-group;music;mathematics;periodic-table;primes" \
        --metadata="licenseurl:https://creativecommons.org/licenses/by/4.0/" \
        --metadata="creator:Meta-Introspector"
    echo "✅ Musical Periodic Table uploaded"
else
    echo "⚠️  Musical Periodic Table not found"
fi

# Generate checksums
echo ""
echo "🔐 Generating checksums..."
cat > checksums.txt << 'EOF'
# SHA256 checksums for Monster OSM Quest data
# Generated: $(date -u +%Y-%m-%dT%H:%M:%SZ)

EOF

for file in /mnt/data1/spool/experiments_monster/*.json; do
    if [ -f "$file" ]; then
        sha256sum "$file" >> checksums.txt
    fi
done

echo "✅ Checksums generated"

echo ""
echo "📊 Upload Summary:"
echo "  - LMFDB Monster 71 Shards"
echo "  - Ramanujan OSM Locations"
echo "  - Musical Periodic Table"
echo ""
echo "🌐 View at: https://archive.org/details/@meta_introspector"
