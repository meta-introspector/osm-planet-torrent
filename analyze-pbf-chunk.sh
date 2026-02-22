#!/usr/bin/env bash
# Analyze PBF chunk structure

echo "🔍 Analyzing PBF chunk structure"
echo ""

# PBF format:
# - File starts with BlobHeader
# - BlobHeader: size (varint) + type + datasize
# - Blob: compressed data (zlib)

CHUNK="/tmp/chunk_13668.raw"

echo "=== Looking for PBF patterns ==="
echo ""

# Look for "OSMHeader" or "OSMData" strings
echo "Searching for PBF markers:"
strings $CHUNK | grep -E "OSM|Data|Header" | head -5
echo ""

# Look for zlib magic (0x78 0x9c or 0x78 0xda)
echo "Searching for zlib compressed blocks:"
xxd $CHUNK | grep -E "789c|78da" | head -5
echo ""

# Try osmium tool if available
if command -v osmium &> /dev/null; then
    echo "=== Using osmium to analyze ==="
    osmium fileinfo $CHUNK 2>&1 | head -20
else
    echo "osmium not available"
fi
echo ""

# Summary
echo "=== ANALYSIS ==="
echo "Chunk appears to be mid-stream PBF data"
echo "Need to:"
echo "  1. Find blob boundaries"
echo "  2. Decompress zlib blocks"
echo "  3. Parse protobuf messages"
echo ""
echo "Alternative: Use piece boundaries from torrent"
echo "  - Torrent pieces are aligned"
echo "  - Fetch complete piece (4MB)"
echo "  - Guaranteed valid PBF structure"
