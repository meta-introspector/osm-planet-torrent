#!/usr/bin/env bash
set -euo pipefail

DATASETS=(
  "geo_shards:1.1M"
  "monster_shards:1.2M"
  "ramanujan_tiles:4.1M"
  "chunks:41M"
  "index:87M"
  "shards:1.0G"
  "tiles-T2:2.5M"
  "tiles-T3:2.5M"
  "tiles-T5:2.5M"
  "tiles-T7:2.5M"
  "tiles-T11:2.5M"
  "tiles-T13:2.5M"
  "tiles-T17:2.5M"
  "tiles-T19:2.5M"
  "tiles-T23:2.5M"
  "tiles-T29:2.5M"
  "tiles-T31:2.5M"
  "tiles-T41:2.5M"
  "tiles-T47:2.5M"
  "tiles-T59:2.5M"
  "tiles-T71:2.5M"
)

for dataset in "${DATASETS[@]}"; do
  IFS=: read -r dir size <<< "$dataset"
  
  echo ""
  echo "📤 Uploading $dir ($size) to Archive.org..."
  
  ia upload "osm-planet-${dir}-monster" \
    "${dir}/" \
    --no-derive \
    --retries 3 \
    || echo "⚠️  Failed: $dir"
  
  echo "✅ Done: $dir"
done

echo ""
echo "🎉 All data uploaded!"
