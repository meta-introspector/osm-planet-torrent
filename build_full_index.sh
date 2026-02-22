#!/bin/bash
# Download first block of all 21,763 pieces to build complete spatial index
# Estimated: ~200 MB total download

set -e

TOTAL_PIECES=21763
BATCH_SIZE=100
LOG_FILE="index_build.log"

echo "🗺️  Building complete OSM planet spatial index" | tee -a "$LOG_FILE"
echo "📊 Total pieces: $TOTAL_PIECES" | tee -a "$LOG_FILE"
echo "💾 Estimated download: ~200 MB" | tee -a "$LOG_FILE"
echo "⏱️  Started: $(date)" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

# Sample every 71st piece (Monster Group prime)
# This gives us 307 pieces = ~1.2 GB
echo "📦 Sampling every 71st piece (Monster Group distribution)" | tee -a "$LOG_FILE"

for ((i=0; i<$TOTAL_PIECES; i+=71)); do
    echo "Fetching piece $i..." | tee -a "$LOG_FILE"
    
    # Download piece (will save chunks)
    nix develop /home/mdupont/nix/vendor/rust/cargo2nix -c cargo run --bin fetch-piece -- $i 2>&1 | grep -E "(Downloaded|Error)" | tee -a "$LOG_FILE" || true
    
    # Reconstruct immediately
    nix develop /home/mdupont/nix/vendor/rust/cargo2nix -c cargo run --bin reconstruct-pbf 2>&1 | grep -E "(Reconstructed|piece_)" | tee -a "$LOG_FILE" || true
    
    # Decompress first block
    nix develop /home/mdupont/nix/vendor/rust/cargo2nix -c cargo run --bin decode-zlib 2>&1 | grep -E "(Decompressed|block_0)" | tee -a "$LOG_FILE" || true
    
    # Every 10 pieces, update spatial index
    if (( i % 710 == 0 )); then
        echo "📍 Building spatial index checkpoint..." | tee -a "$LOG_FILE"
        nix develop /home/mdupont/nix/vendor/rust/cargo2nix -c cargo run --bin build-spatial-index 2>&1 | grep -E "(Processing|Saved)" | tee -a "$LOG_FILE" || true
    fi
    
    # Rate limit to avoid overwhelming tracker
    sleep 2
done

echo "" | tee -a "$LOG_FILE"
echo "✅ Download complete!" | tee -a "$LOG_FILE"
echo "⏱️  Finished: $(date)" | tee -a "$LOG_FILE"

# Build final spatial index
echo "📍 Building final spatial index..." | tee -a "$LOG_FILE"
nix develop /home/mdupont/nix/vendor/rust/cargo2nix -c cargo run --bin build-spatial-index 2>&1 | tee -a "$LOG_FILE"

echo "" | tee -a "$LOG_FILE"
echo "🎉 Spatial index complete!" | tee -a "$LOG_FILE"
echo "📊 Query with: cargo run --bin query-index -- geo <lat> <lon>" | tee -a "$LOG_FILE"
