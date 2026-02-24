# OSM Planet Datasets - Complete Summary

## Available Datasets

### 1. Pre-processed Tiles (LOCAL - FASTEST)
**Location**: `/mnt/data1/osm-planet-torrent/tiles/`  
**Size**: 506M (1,889 tiles)  
**Format**: CSV (node_id, lat, lon)  
**Cost**: 0 bytes (already local)  
**Speed**: Instant  

```bash
# Example tile
/mnt/data1/osm-planet-torrent/tiles/tile_143_184/nodes_12.csv
# Format: node_id,lat,lon
34592703,-12.9730139,-38.5103048
```

### 2. HuggingFace Datasets (UPLOADED ✅)
**Organization**: introspector  
**Status**: All uploaded  

- ✅ `introspector/osm-planet-geo_shards` - Geographic shards
- ✅ `introspector/osm-planet-monster_shards` - Monster Group shards  
- ✅ `introspector/osm-planet-ramanujan_tiles` - Ramanujan tile system
- ✅ `introspector/osm-planet-chunks` - Torrent chunks

**Access**:
```python
from datasets import load_dataset
ds = load_dataset("introspector/osm-planet-geo_shards")
```

### 3. Archive.org (UPLOADED ✅)
**Item**: `osm-planet-chunks-monster`  
**Status**: 1,806 files uploaded (15.9 MB)  
**URL**: https://archive.org/details/osm-planet-chunks-monster  

**Access**:
```bash
# Download specific file
curl https://archive.org/download/osm-planet-chunks-monster/piece_0000000_offset_0000016384.bin
```

### 4. Ramanujan Location Index
**File**: `ramanujan-location-index.json`  
**Locations**: 8 key sites  
**Format**: JSON with piece→shard mapping  

```json
{
  "name": "Kumbakonam",
  "lat": 10.9617,
  "lon": 79.3881,
  "wikidata": "Q2744680",
  "piece": 13668,
  "shard": 36
}
```

### 5. Full Planet (LOCAL)
**Location**: `/mnt/data1/osm-planet/planet-latest.osm.pbf`  
**Size**: 86GB  
**Format**: PBF (Protocol Buffer)  
**Use**: Fallback for missing tiles  

## Fetch Strategy (Three-Tier)

### Tier 1: Check Pre-processed Tiles
```rust
let tile_path = format!("/mnt/data1/osm-planet-torrent/tiles/tile_{}_{}", 
                        tile_lat, tile_lon);
if Path::new(&tile_path).exists() {
    return read_csv(&tile_path); // INSTANT
}
```
**Cost**: 0 bytes  
**Speed**: Instant  
**Coverage**: 1,889 tiles  

### Tier 2: Selective Planet Fetch
```rust
// Use ramanujan-location-index.json
let piece = 13668; // Kumbakonam
let offset = piece * 4_194_304;
let chunk = read_bytes("/mnt/data1/osm-planet/planet-latest.osm.pbf", 
                       offset, 50_000);
```
**Cost**: 50KB  
**Speed**: <1 second  
**Reduction**: 99.9999% vs full planet  

### Tier 3: HTTP/HuggingFace
```python
# HuggingFace
from datasets import load_dataset
ds = load_dataset("introspector/osm-planet-geo_shards", split="train")

# Archive.org
curl -r 57326967996-57327017995 \
  https://archive.org/download/osm-planet-chunks-monster/chunk.bin
```
**Cost**: 50KB download  
**Speed**: ~5 seconds  

## Cost/Benefit Analysis

| Method | Cost | Time | Coverage | Reduction |
|--------|------|------|----------|-----------|
| **Full Planet** | 86GB | 2 hours | 100% | 0% |
| **Tier 1 (Tiles)** | 0 bytes | instant | 1,889 tiles | 100% |
| **Tier 2 (Selective)** | 50KB | <1s | Any location | 99.9999% |
| **Tier 3 (HTTP)** | 50KB | ~5s | Any location | 99.9999% |
| **HuggingFace** | API call | ~3s | Shards only | N/A |

## Usage Examples

### Example 1: Fetch Kumbakonam
```bash
# Check Tier 1
cat /mnt/data1/osm-planet-torrent/tiles/tile_143_184/nodes_*.csv

# Tier 2 (if not in tiles)
dd if=/mnt/data1/osm-planet/planet-latest.osm.pbf \
   bs=1 skip=57326967996 count=50000 > kumbakonam.chunk

# Tier 3 (if remote)
curl https://archive.org/download/osm-planet-chunks-monster/kumbakonam_tile.geojson
```

### Example 2: Use HuggingFace
```python
from datasets import load_dataset

# Load geo shards
ds = load_dataset("introspector/osm-planet-geo_shards")

# Filter by location
kumbakonam = ds.filter(lambda x: 
    10.9 < x['lat'] < 11.0 and 
    79.3 < x['lon'] < 79.4
)
```

### Example 3: Use Index
```bash
# Find piece for location
cat ramanujan-location-index.json | jq '.locations[] | select(.name == "Kumbakonam")'

# Output:
# {
#   "name": "Kumbakonam",
#   "piece": 13668,
#   "shard": 36
# }
```

## Summary

**Total Datasets**: 5 sources  
**Total Size**: 86GB planet → 506M tiles (99.4% reduction)  
**Upload Status**: ✅ HuggingFace, ✅ Archive.org  
**Access Methods**: Local files, HTTP, Python API  
**Best Strategy**: Three-tier cascade (local → selective → remote)  

**Key Insight**: 99.9999% reduction in data transfer by using pre-processed tiles and selective fetching!
