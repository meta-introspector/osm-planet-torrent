# OSM Ramanujan Tiles - Hosting Plan

## Data Distribution

### HuggingFace (Primary)
```
https://huggingface.co/datasets/meta-introspector/osm-ramanujan-tiles
```
- 196,883 tiles × 344 KB (gzipped) = ~68 GB total
- Free hosting for datasets
- Fast CDN delivery
- Version control with Git LFS

### Archive.org (Mirror)
```
https://archive.org/download/osm-ramanujan-tiles/
```
- Permanent archival
- No bandwidth limits
- Public domain friendly

### GitHub Pages (Index Only)
```
https://meta-introspector.github.io/osm-planet-torrent/
```
- Interactive map
- Tile calculator
- Query interface
- NO data files (just HTML/JS)

## Upload Process

### 1. Compress tiles
```bash
cd ramanujan_tiles
for f in *.csv; do
    gzip -9 "$f"
done
```

### 2. Upload to HuggingFace
```bash
# Install huggingface-cli
pip install huggingface_hub

# Login
huggingface-cli login

# Create dataset
huggingface-cli repo create osm-ramanujan-tiles --type dataset

# Upload tiles
cd ramanujan_tiles
huggingface-cli upload meta-introspector/osm-ramanujan-tiles . .
```

### 3. Upload to Archive.org
```bash
# Install internetarchive
pip install internetarchive

# Configure
ia configure

# Upload
ia upload osm-ramanujan-tiles ramanujan_tiles/*.csv.gz \
  --metadata="title:OSM Ramanujan Tiles" \
  --metadata="description:OpenStreetMap planet sharded by Ramanujan primes (71×59×47)" \
  --metadata="creator:meta-introspector" \
  --metadata="licenseurl:https://opendatacommons.org/licenses/odbl/"
```

## Tile Naming Convention
```
tile_LL_OO_HH.csv.gz

LL = latitude bucket (00-70)
OO = longitude bucket (00-58)
HH = height/level bucket (00-46)
```

## Query Examples

### Kumbakonam (10.9617°N, 79.3881°E)
```
tile_lat = int(((10.9617 + 90) * 100) % 71) = 14
tile_lon = int(((79.3881 + 180) * 100) % 59) = 37
tile_level = 0

URL: tile_14_37_00.csv.gz
```

### London (51.5074°N, -0.1278°E)
```
tile_lat = 25
tile_lon = 30
tile_level = 0

URL: tile_25_30_00.csv.gz
```

## Bandwidth Estimates

- Queries per day: 1,000
- Avg tile size: 344 KB
- Daily bandwidth: 344 MB
- Monthly: ~10 GB

Both HuggingFace and Archive.org can handle this easily.
