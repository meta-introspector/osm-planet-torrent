#!/usr/bin/env bash
set -euo pipefail

DATASETS=(
  "shards:Monster Group shards (71×59 grid)"
  "chunks:Downloaded torrent chunks"
  "index:Piece index"
  "ramanujan_tiles:Ramanujan-specific tiles"
  "monster_shards:Monster Group shards"
  "tiles-T2:Hecke operator T_2"
  "tiles-T3:Hecke operator T_3"
  "tiles-T5:Hecke operator T_5"
  "tiles-T7:Hecke operator T_7"
  "tiles-T11:Hecke operator T_11"
  "tiles-T13:Hecke operator T_13"
  "tiles-T17:Hecke operator T_17"
  "tiles-T19:Hecke operator T_19"
  "tiles-T23:Hecke operator T_23"
  "tiles-T29:Hecke operator T_29"
  "tiles-T31:Hecke operator T_31"
  "tiles-T41:Hecke operator T_41"
  "tiles-T47:Hecke operator T_47"
  "tiles-T59:Hecke operator T_59"
  "tiles-T71:Hecke operator T_71"
)

for dataset in "${DATASETS[@]}"; do
  IFS=: read -r dir desc <<< "$dataset"
  
  echo ""
  echo "📤 Uploading $dir to Archive.org..."
  
  ia upload "osm-planet-${dir}-monster" \
    "${dir}/README.md" \
    "${dir}/FILE_LIST.txt" \
    --metadata="title:OSM Planet Torrent - ${desc}" \
    --metadata="description:${desc} using Monster Group symmetries [71,59,47]" \
    --metadata="subject:OpenStreetMap" \
    --metadata="subject:Monster Group" \
    --metadata="creator:Meta-Introspector" \
    --metadata="date:2026-02-22" \
    --metadata="licenseurl:https://opendatacommons.org/licenses/odbl/1-0/" \
    || echo "⚠️  Failed: $dir"
  
  echo "✅ Done: https://archive.org/details/osm-planet-${dir}-monster"
done

echo ""
echo "🎉 All uploads complete!"
