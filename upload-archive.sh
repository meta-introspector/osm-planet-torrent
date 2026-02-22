#!/usr/bin/env bash
# Upload datasets to Archive.org first, then HuggingFace

set -euo pipefail

# Check for internetarchive CLI
if ! command -v ia &> /dev/null; then
    echo "📦 Installing internetarchive CLI..."
    nix-shell -p python3Packages.internetarchive --run "ia --version"
fi

# Check if configured
if ! ia configure --check &>/dev/null; then
    echo "🔐 Configure Archive.org credentials:"
    echo "   ia configure"
    exit 1
fi

DATASETS=(
    "shards:Monster Group shards (71×59 grid)"
    "tiles:OSM tiles (520K files)"
    "chunks:Downloaded torrent chunks"
    "index:Piece index (901K files)"
    "ramanujan_tiles:Ramanujan-specific tiles"
    "monster_shards:Monster Group shards"
    "geo_shards:Geographic shards"
)

for dataset in "${DATASETS[@]}"; do
    IFS=: read -r dir desc <<< "$dataset"
    
    if [ ! -d "$dir" ]; then
        echo "⏭️  Skipping $dir (not found)"
        continue
    fi
    
    echo ""
    echo "📤 Uploading $dir to Archive.org..."
    
    # Create identifier
    IDENTIFIER="osm-planet-$dir-monster-group"
    
    # Upload to Archive.org
    nix-shell -p python3Packages.internetarchive --run "ia upload $IDENTIFIER \
        $dir/ \
        --metadata='title:OSM Planet Torrent - $desc' \
        --metadata='description:Part of Monster OSM Quest. Sharded by Monster Group (71×59×47). Source: OpenStreetMap Planet via BitTorrent.' \
        --metadata='subject:OpenStreetMap' \
        --metadata='subject:Monster Group' \
        --metadata='subject:Geographic Data' \
        --metadata='subject:Ramanujan' \
        --metadata='creator:Meta-Introspector' \
        --metadata='licenseurl:https://opendatacommons.org/licenses/odbl/1-0/' \
        # --metadata='collection:opensource_data'  # Requires collection privileges" || echo "⚠️  Archive.org upload failed for $dir"
    
    echo "✅ $dir uploaded to Archive.org: https://archive.org/details/$IDENTIFIER"
done

echo ""
echo "✅ All datasets uploaded to Archive.org"
echo ""
echo "Next: Upload to HuggingFace with ./upload-datasets.sh"
