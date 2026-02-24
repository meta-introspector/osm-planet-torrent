#!/usr/bin/env bash
# Split tiles/ into 71 Monster shards

set -euo pipefail

TILES_DIR="tiles"
OUTPUT_BASE="tiles-shards"

echo "📊 Splitting $TILES_DIR into 71 shards..."

# Create output directory
mkdir -p "$OUTPUT_BASE"

# Read file list
TOTAL=$(wc -l < "$TILES_DIR/FILE_LIST.txt")
echo "Total files: $TOTAL"

# Calculate files per shard
PER_SHARD=$((TOTAL / 71))
echo "Files per shard: ~$PER_SHARD"

# Split into 71 shards
split -l $PER_SHARD -d -a 2 "$TILES_DIR/FILE_LIST.txt" "$OUTPUT_BASE/shard-"

# Rename to 00-70
for i in {0..70}; do
    SHARD_NUM=$(printf "%02d" $i)
    if [ -f "$OUTPUT_BASE/shard-$SHARD_NUM" ]; then
        mv "$OUTPUT_BASE/shard-$SHARD_NUM" "$OUTPUT_BASE/shard-$SHARD_NUM.txt"
        echo "✅ Shard $SHARD_NUM: $(wc -l < "$OUTPUT_BASE/shard-$SHARD_NUM.txt") files"
    fi
done

echo ""
echo "✅ Split complete: 71 shards in $OUTPUT_BASE/"
echo ""
echo "Next: Create 71 separate datasets"
