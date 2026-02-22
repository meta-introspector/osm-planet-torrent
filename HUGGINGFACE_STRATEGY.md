# OSM Planet → HuggingFace Dataset Strategy

## Phase 1: Download Full Planet (85 GB)

Use librqbit to download complete file (fast with good peers):

```bash
# Download full planet file
cargo run -- --torrent-url "https://planet.openstreetmap.org/torrent/planet-latest.osm.pbf.torrent"
```

This will be MUCH faster than piece-by-piece because:
- More peers available for complete file
- Sequential download is optimized
- No per-piece overhead

## Phase 2: Index the Full File

Once downloaded, scan the entire file and build complete index:

```rust
// Read planet file sequentially
// For each PBF blob:
//   - Extract node ID range
//   - Extract bounding box
//   - Calculate Monster geo block
//   - Count wikidata entities
//   - Record byte offset and length

// Output: complete_spatial_index.parquet
```

**Index size:** ~50 MB (21,763 pieces × 2 KB metadata)

## Phase 3: Create HuggingFace Dataset

Upload to HuggingFace:

```
osm-planet-spatial-index/
├── README.md                    # Dataset card
├── spatial_index.parquet        # Complete index
├── monster_geo_blocks.json      # 71×59 grid mapping
├── node_id_ranges.parquet       # Fast node ID lookup
└── sample_pieces/               # First 10 pieces as examples
    ├── piece_0000000.pbf
    ├── piece_0000001.pbf
    └── ...
```

## Phase 4: Query Interface

Users can:

1. **Download just the index** (~50 MB)
2. **Query locally** for their region
3. **Download specific pieces** from original torrent/HTTP
4. **Or use our pre-sliced chunks** on HuggingFace

## Phase 5: Share Sliced Chunks (Optional)

For popular regions, upload pre-extracted chunks:

```
osm-planet-chunks/
├── india/
│   ├── kumbakonam_nodes.parquet
│   ├── kumbakonam_ways.parquet
│   └── kumbakonam_relations.parquet
├── usa/
├── europe/
└── ...
```

## Benefits

✅ **Fast initial download** - Full torrent with good peers
✅ **Complete index** - All 21,763 pieces mapped
✅ **Shareable** - Anyone can query without downloading 85 GB
✅ **Reproducible** - Open dataset on HuggingFace
✅ **Efficient** - Download only needed pieces
✅ **Monster Group** - 71×59 geo grid for efficient spatial queries

## Implementation

```bash
# 1. Download full planet
./download_full_planet.sh

# 2. Build complete index
cargo run --bin index-full-planet -- planet-latest.osm.pbf

# 3. Upload to HuggingFace
huggingface-cli upload osm-planet-spatial-index ./spatial_index.parquet

# 4. Share usage instructions
```

## Dataset Card (README.md)

```markdown
# OSM Planet Spatial Index

Complete spatial index of OpenStreetMap planet file enabling efficient 
geographic queries without downloading 85 GB.

## Usage

```python
from datasets import load_dataset
import pandas as pd

# Load index
index = load_dataset("username/osm-planet-spatial-index")
df = pd.DataFrame(index['train'])

# Find pieces for Kumbakonam, India
pieces = df[
    (df['min_lat'] <= 10.9617) & (df['max_lat'] >= 10.9617) &
    (df['min_lon'] <= 79.3881) & (df['max_lon'] >= 79.3881)
]

# Download specific pieces
for piece_id in pieces['piece_id']:
    download_piece(piece_id)
```

## Monster Group Geo Grid

World divided into 71×59 = 4,189 geographic blocks using Monster Group primes.

## License

ODbL (OpenStreetMap data license)
```

## Next Steps

1. Start full planet download via torrent
2. Build indexing tool that scans complete file
3. Generate parquet index with all metadata
4. Create HuggingFace dataset
5. Share with community!
