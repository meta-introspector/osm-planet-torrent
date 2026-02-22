#!/usr/bin/env bash
# Initialize large data directories as HuggingFace datasets

set -euo pipefail

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
    
    echo "📦 Initializing $dir as HuggingFace dataset..."
    
    cd "$dir"
    
    # Git init
    git init
    
    # Create README
    cat > README.md << EOF
# OSM Planet Torrent - $desc

Part of the Monster OSM Quest project.

## Dataset Info

- **Source**: OpenStreetMap Planet via BitTorrent
- **Sharding**: Monster Group (71×59×47)
- **Format**: PBF/Parquet
- **License**: ODbL (OpenStreetMap)

## Monster Symmetries

- Input: [71, 59, 47] (Keter/Binah/Chokmah)
- Output: [17, 23, 59] (Cusp/Consciousness/Memory)
- Invariants: geographic, torrent, Monster-Group, OSM

## Usage

\`\`\`python
from datasets import load_dataset
dataset = load_dataset("meta-introspector/osm-planet-$dir")
\`\`\`

## Parent Project

https://github.com/meta-introspector/osm-planet-torrent
EOF
    
    # Create .gitattributes for LFS
    cat > .gitattributes << EOF
*.pbf filter=lfs diff=lfs merge=lfs -text
*.parquet filter=lfs diff=lfs merge=lfs -text
*.bin filter=lfs diff=lfs merge=lfs -text
EOF
    
    # Add files
    git add README.md .gitattributes
    git commit -m "Initialize $desc dataset"
    
    echo "✅ $dir initialized"
    cd ..
done

echo ""
echo "✅ All datasets initialized"
echo ""
echo "Next steps:"
echo "  1. Install HuggingFace CLI: pip install huggingface_hub"
echo "  2. Login: huggingface-cli login"
echo "  3. Create repos on HuggingFace"
echo "  4. Push each dataset:"
for dataset in "${DATASETS[@]}"; do
    IFS=: read -r dir desc <<< "$dataset"
    [ -d "$dir" ] && echo "     cd $dir && git remote add origin https://huggingface.co/datasets/introspector/osm-planet-$dir && git push -u origin main"
done
