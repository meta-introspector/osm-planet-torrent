#!/usr/bin/env bash
# Create 15 Hecke operator datasets

set -euo pipefail

HECKE_DIR="tiles-hecke"
PRIMES=(2 3 5 7 11 13 17 19 23 29 31 41 47 59 71)

for p in "${PRIMES[@]}"; do
    DATASET="tiles-T$p"
    
    echo "📦 Creating $DATASET..."
    
    mkdir -p "$DATASET"
    cd "$DATASET"
    
    # Git init
    git init
    
    # Create README
    cat > README.md << EOF
# OSM Planet Tiles - Hecke Operator T_$p

Tiles sharded by Hecke operator T_$p (Monster prime $p).

## Dataset Info

- **Hecke Operator**: T_$p
- **Monster Prime**: $p
- **Files**: $(wc -l < "../$HECKE_DIR/T_$p.txt") tiles
- **Sharding**: Hash mod 15 → T_$p
- **Format**: PBF/Parquet
- **License**: ODbL (OpenStreetMap)

## Monster Symmetries

- Input: [71, 59, 47] (Keter/Binah/Chokmah)
- Output: [17, 23, 59] (Cusp/Consciousness/Memory)
- Hecke: T_$p (resonance $p)

## Usage

\`\`\`python
from datasets import load_dataset
dataset = load_dataset("introspector/osm-planet-tiles-T$p")
\`\`\`

## Parent Project

- GitHub: https://github.com/meta-introspector/osm-planet-torrent
- Archive.org: https://archive.org/details/osm-planet-tiles-T$p-monster-group
- HuggingFace: https://huggingface.co/datasets/introspector/osm-planet-tiles-T$p
EOF
    
    # Copy file list
    cp "../$HECKE_DIR/T_$p.txt" FILE_LIST.txt
    
    # Git LFS
    cat > .gitattributes << EOF
*.pbf filter=lfs diff=lfs merge=lfs -text
*.parquet filter=lfs diff=lfs merge=lfs -text
*.bin filter=lfs diff=lfs merge=lfs -text
EOF
    
    # Commit
    git add README.md FILE_LIST.txt .gitattributes
    git commit -m "Initialize Hecke operator T_$p dataset ($(wc -l < FILE_LIST.txt) files)"
    
    cd ..
    echo "✅ $DATASET created"
done

echo ""
echo "✅ All 15 Hecke datasets created"
