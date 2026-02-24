# OSM Tile-Based Sharding Plan

## Concept
Use standard map tile system (Z/X/Y) instead of inventing new sharding.

## Tile System
```
Zoom 0: 1 tile (whole world)
Zoom 1: 4 tiles (2×2)
Zoom 2: 16 tiles (4×4)
...
Zoom N: 4^N tiles (2^N × 2^N)
```

## Hierarchical Structure

### Level 1: Admin Boundaries (Header)
- Download from Geofabrik: admin polygons
- Store in FRACTRAN encoding
- Countries → States → Cities
- Size: ~100 MB compressed

### Level 2: Tile Index (Z=8)
- 256 × 256 = 65,536 tiles
- Each tile: ~1.3 MB (86 GB / 65,536)
- Tile (x, y) = nodes where:
  - lat → y = floor((90 - lat) / 180 * 256)
  - lon → x = floor((lon + 180) / 360 * 256)

### Level 3: Node ID Sharding (within tile)
- Within each tile, split by node_id % 71
- Gives 71 sub-buckets per tile
- Total: 65,536 × 71 = 4,653,056 buckets
- Avg: 18 KB per bucket

### Level 4: Type/Height (optional)
- Further split by type % 7, height % 29
- Only for dense tiles

## Implementation

### Step 1: Download Admin Boundaries
```bash
wget https://www.geofabrik.de/data/admin-polygons.zip
# Parse into FRACTRAN: country_id * state_id * city_id
```

### Step 2: Build Tile Index
```rust
// For each node:
let tile_x = ((lon + 180.0) / 360.0 * 256.0) as u8;
let tile_y = ((90.0 - lat) / 180.0 * 256.0) as u8;
let node_bucket = (node_id % 71) as u8;

// Write to: shards/tile_{x}_{y}/nodes_{bucket}.csv
```

### Step 3: Query by Location
```python
# Kumbakonam: 10.9617°N, 79.3881°E
tile_x = int((79.3881 + 180) / 360 * 256)  # = 184
tile_y = int((90 - 10.9617) / 180 * 256)   # = 113

# Download only: shards/tile_184_113/*.csv
# Size: ~1.3 MB (one tile) instead of 86 GB
```

## Advantages
- Standard tile system (compatible with existing tools)
- Admin boundaries provide semantic structure
- FRACTRAN encoding: country × state × city as single number
- Hierarchical: query admin first, then tiles, then nodes
- Incremental rendering: load tiles as needed

## FRACTRAN Encoding
```
India = 2^356
Tamil Nadu = 3^89
Kumbakonam = 5^71

Location ID = 2^356 × 3^89 × 5^71
```

Query: "Find all nodes in Tamil Nadu"
→ Divisible by 3^89
→ Returns all tiles in that state

## Next Steps
1. Download admin boundaries from Geofabrik
2. Encode admin hierarchy in FRACTRAN
3. Build tile index (Z=8, 65,536 tiles)
4. Test query for Kumbakonam
5. Upload to HuggingFace as dataset
