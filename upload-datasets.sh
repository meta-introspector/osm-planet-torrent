#!/usr/bin/env bash
# Upload datasets to HuggingFace

set -euo pipefail

# Check if logged in
if [ ! -f ~/.cache/huggingface/stored_tokens ]; then
    echo "🔐 Login to HuggingFace first:"
    echo "   nix-shell -p python3Packages.huggingface-hub --run 'hf auth login'"
    exit 1
fi

DATASETS=(
    "shards"
    "tiles"
    "chunks"
    "index"
    "ramanujan_tiles"
    "monster_shards"
    "geo_shards"
)

for dataset in "${DATASETS[@]}"; do
    if [ ! -d "$dataset" ]; then
        echo "⏭️  Skipping $dataset (not found)"
        continue
    fi
    
    echo ""
    echo "📤 Uploading $dataset..."
    
    cd "$dataset"
    
    # Add remote if not exists
    if ! git remote | grep -q origin; then
        git remote add origin "https://huggingface.co/datasets/introspector/osm-planet-$dataset"
    fi
    
    # Push
    git push -u origin main || echo "⚠️  Push failed for $dataset"
    
    cd ..
    
    echo "✅ $dataset uploaded"
done

echo ""
echo "✅ All datasets uploaded to HuggingFace"
echo ""
echo "View at:"
for dataset in "${DATASETS[@]}"; do
    [ -d "$dataset" ] && echo "  https://huggingface.co/datasets/meta-introspector/osm-planet-$dataset"
done
