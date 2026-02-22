# Complete Spatial Index Strategy

## Goal
Build a complete index of the 85 GB OSM planet file by downloading ONLY ~200 MB of metadata.

## Architecture

### 1. Monster Group Geo Grid (71 × 59 = 4,189 blocks)

**Latitude blocks (71):**
- Range: -90° to +90° (180°)
- Block size: 180° / 71 = 2.535° per block
- Example: Block 39 = 8.87° to 11.41° (contains Kumbakonam at 10.96°)

**Longitude blocks (59):**
- Range: -180° to +180° (360°)
- Block size: 360° / 59 = 6.102° per block
- Example: Block 42 = 76.27° to 82.37° (contains Kumbakonam at 79.39°)

**Kumbakonam location:**
- Coordinates: 10.9617°N, 79.3881°E
- Monster block: (39, 42)

### 2. Three-Level Index

**Level 1: Piece → Node Range**
```json
{
  "piece_id": 1,
  "min_node_id": 20933784,
  "max_node_id": 21458266,
  "node_count": 524482
}
```

**Level 2: Piece → Bounding Box**
```json
{
  "piece_id": 1,
  "min_lat": -35.3963,
  "max_lat": 69.8251,
  "min_lon": -157.9457,
  "max_lon": 153.5761
}
```

**Level 3: Piece → Monster Geo Block**
```json
{
  "piece_id": 1,
  "monster_lat_block": 42,
  "monster_lon_block": 29,
  "wikidata_count": 47
}
```

### 3. Index Building Process

**Phase 1: Download first block of each piece** (~200 MB total)
```bash
# Download every piece's first block (just metadata)
for piece_id in 0..21762; do
  cargo run --bin fetch-piece -- $piece_id --first-block-only
done
```

**Phase 2: Extract metadata from each block**
```bash
# Parse each block and extract:
# - min/max node IDs
# - min/max lat/lon
# - wikidata entity count
cargo run --bin build-spatial-index
```

**Phase 3: Calculate Monster geo blocks**
```rust
monster_lat_block = ((lat + 90.0) / 180.0 * 71.0) as u8;
monster_lon_block = ((lon + 180.0) / 360.0 * 59.0) as u8;
```

### 4. Query Patterns

**By Location:**
```bash
cargo run --bin query-index -- geo 10.9617 79.3881
# Returns: All pieces containing this location
```

**By Node ID:**
```bash
cargo run --bin query-index -- node 2824755486
# Returns: Piece containing this node
```

**By Monster Geo Block:**
```bash
cargo run --bin query-index -- monster 10.9617 79.3881
# Returns: Monster block (39, 42) and all pieces in it
```

### 5. On-Demand Download

Once index is built:
```bash
# 1. Query for target location
cargo run --bin query-index -- geo 10.9617 79.3881

# 2. Download specific pieces (40-200 MB)
cargo run --bin fetch-piece -- 1234
cargo run --bin fetch-piece -- 5678

# 3. Extract wikidata entities
cargo run --bin parse-dense
```

## Storage Requirements

**Without index:** 85 GB (full planet file)

**With index:**
- Index metadata: ~200 MB (first block of each piece)
- Spatial index JSON: ~5 MB (21,763 pieces × 200 bytes)
- On-demand pieces: 40-200 MB (10-50 pieces for target area)
- **Total: ~250 MB** (0.3% of full file!)

## Monster Group Properties

**Total geo blocks:** 71 × 59 = 4,189
**Sharding mod:** 71 × 59 × 497 = 2,081,933
**Pieces per block:** 21,763 / 4,189 ≈ 5.2 pieces/block (average)

**Distribution:**
- Some blocks have 0 pieces (oceans)
- Some blocks have 50+ pieces (dense cities)
- Monster arithmetic enables efficient sharding

## Next Steps

1. Modify `fetch-piece` to support `--first-block-only` flag
2. Download first block of all 21,763 pieces (~200 MB)
3. Build complete spatial index
4. Query for Kumbakonam area (Monster block 39, 42)
5. Download relevant pieces
6. Extract all Wikidata Q IDs
