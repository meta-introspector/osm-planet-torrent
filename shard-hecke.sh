#!/usr/bin/env bash
# Split tiles/ by Hecke operator T_p (15 Monster primes)

set -euo pipefail

TILES_DIR="tiles"
OUTPUT_BASE="tiles-hecke"

# 15 Monster primes (Hecke operators)
PRIMES=(2 3 5 7 11 13 17 19 23 29 31 41 47 59 71)

echo "📊 Sharding $TILES_DIR by Hecke operators T_p..."
echo "Primes: ${PRIMES[*]}"

# Create output directory
mkdir -p "$OUTPUT_BASE"

# Read file list
TOTAL=$(wc -l < "$TILES_DIR/FILE_LIST.txt")
echo "Total files: $TOTAL"

# Shard by hash mod 15
awk -v base="$OUTPUT_BASE" '
BEGIN {
    primes[0]=2; primes[1]=3; primes[2]=5; primes[3]=7; primes[4]=11;
    primes[5]=13; primes[6]=17; primes[7]=19; primes[8]=23; primes[9]=29;
    primes[10]=31; primes[11]=41; primes[12]=47; primes[13]=59; primes[14]=71;
}
{
    # Hash filename
    hash = 0;
    for (i=1; i<=length($0); i++) {
        hash = (hash * 31 + ord(substr($0, i, 1))) % 1000000007;
    }
    
    # Assign to Hecke operator (mod 15)
    shard = hash % 15;
    prime = primes[shard];
    
    # Write to shard file
    file = sprintf("%s/T_%d.txt", base, prime);
    print $0 >> file;
}

function ord(c) {
    return index("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-./", c);
}
' "$TILES_DIR/FILE_LIST.txt"

# Report
echo ""
echo "✅ Hecke sharding complete:"
for p in "${PRIMES[@]}"; do
    if [ -f "$OUTPUT_BASE/T_$p.txt" ]; then
        COUNT=$(wc -l < "$OUTPUT_BASE/T_$p.txt")
        echo "   T_$p: $COUNT files"
    fi
done

echo ""
echo "Next: Create 15 Hecke datasets"
