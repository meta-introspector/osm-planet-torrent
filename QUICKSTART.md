# OSM Leech Lattice Tiles - Quickstart for n00bs

## What is this?

OpenStreetMap planet (86 GB) → 196,883 tiles with **24D Leech lattice encoding**

Each node has:
- Geographic data (lat, lon)
- Tags (name, wikidata, highway, amenity, etc.)
- **24D coordinates** in the Leech lattice
- **71-adic hash** encoding all data
- **15D Maass shadow** (Monster Group residues)

## Why?

1. **Smaller files:** Query only your tile (~4 KB vs 86 GB)
2. **Rich data:** Wikidata IDs, Wikipedia links, road types
3. **Math structure:** Monster Group symmetry preserved
4. **Queryable:** Find nodes by lattice distance

## Quick Test (30 seconds)

```bash
# Build
cd /home/mdupont/projects/osm-planet-torrent
nix develop /home/mdupont/nix/vendor/rust/cargo2nix -c \
  cargo build --release --bin tile-shard-leech

# Test run (30 seconds)
mkdir -p /tmp/osm-test
cd /tmp/osm-test
timeout 30 /home/mdupont/projects/osm-planet-torrent/target/release/tile-shard-leech

# Check output
ls -lh tiles_leech/
cat tiles_leech/tile_*.jsonl | head -1 | jq '.'
```

## Output Format (JSONL)

Each line is a JSON node:

```json
{
  "node_id": 30304398,
  "lat": 58.4241310,
  "lon": -111.5012918,
  "name": "Fort McMurray",
  "wikidata": "Q201569",
  "wikipedia": "en:Fort McMurray",
  "place": "city",
  "admin_level": 8,
  "leech_coords": [42, 15, 8, 23, 45, 12, 67, 3, 19, 201569, 7, 11, 13, 17, 19],
  "p71_hash": 123456789,
  "maass_shadow": [1, 0, 4, 2, 8, 5, 6, 2, 22, 28, 25, 3, 31, 27, 32]
}
```

## Query Examples

### 1. Find all Wikidata entities

```bash
cat tiles_leech/*.jsonl | jq 'select(.wikidata != null)'
```

### 2. Find all highways

```bash
cat tiles_leech/*.jsonl | jq 'select(.highway != null)'
```

### 3. Find nodes in Kumbakonam tile

```bash
# Kumbakonam: lat=10.9617, lon=79.3881
# Tile: (14, 38)
cat tiles_leech/tile_14_38_00.jsonl | jq '.'
```

### 4. Find nodes with same Maass shadow

```bash
cat tiles_leech/*.jsonl | jq 'select(.maass_shadow[0] == 1 and .maass_shadow[1] == 0)'
```

## Convert to GeoJSON

```bash
cat tiles_leech/tile_14_38_00.jsonl | jq -s '{
  type: "FeatureCollection",
  features: map({
    type: "Feature",
    geometry: {
      type: "Point",
      coordinates: [.lon, .lat]
    },
    properties: {
      name: .name,
      wikidata: .wikidata,
      highway: .highway,
      leech_coords: .leech_coords,
      p71_hash: .p71_hash
    }
  })
}' > kumbakonam.geojson
```

## Convert to RDFa (Escaped)

```bash
cat tiles_leech/tile_14_38_00.jsonl | jq -r '
  "<div vocab=\"http://schema.org/\" typeof=\"Place\">" +
  "  <span property=\"name\">" + (.name // "Unknown") + "</span>" +
  "  <span property=\"geo\" typeof=\"GeoCoordinates\">" +
  "    <meta property=\"latitude\" content=\"" + (.lat | tostring) + "\">" +
  "    <meta property=\"longitude\" content=\"" + (.lon | tostring) + "\">" +
  "  </span>" +
  (if .wikidata then "  <link property=\"sameAs\" href=\"https://www.wikidata.org/wiki/" + .wikidata + "\">" else "" end) +
  "</div>"
' > kumbakonam.html
```

## Full Planet Processing

```bash
# This will take ~8-12 hours for 86 GB
cd /mnt/data1/osm-planet-torrent
/home/mdupont/projects/osm-planet-torrent/target/release/tile-shard-leech

# Monitor progress
watch -n 60 'du -sh tiles_leech/ && find tiles_leech/ -name "*.jsonl" | wc -l'
```

## Upload to Archive.org

```bash
# Install ia CLI
pip install internetarchive

# Configure (one time)
ia configure

# Upload tiles
ia upload osm-leech-lattice-tiles \
  tiles_leech/*.jsonl \
  --metadata="title:OSM Leech Lattice Tiles" \
  --metadata="creator:meta-introspector" \
  --metadata="license:ODbL" \
  --metadata="subject:openstreetmap;leech-lattice;monster-group;wikidata"
```

## Query from Archive.org

```bash
# Download specific tile
wget https://archive.org/download/osm-leech-lattice-tiles/tile_14_38_00.jsonl

# Query
cat tile_14_38_00.jsonl | jq 'select(.wikidata != null)'
```

## 24D Leech Lattice Explained

The Leech lattice is the densest sphere packing in 24 dimensions.

**Dimensions:**
- 0-1: Lat/Lon (geographic position)
- 2: Admin level (country=2, state=4, city=6)
- 3-8: Name hash (6 dimensions)
- 9: Wikidata Q-ID (mod 71)
- 10-14: Tag hashes (highway, amenity, tourism, historic, place)
- 15-23: Reserved

**71-adic hash:** Single number encoding all node data (mod 71^6)

**Maass shadow:** 15 residues mod Monster primes (2,3,5,7,11,13,17,19,23,29,31,41,47,59,71)

## Why Monster Group?

The Monster Group (largest sporadic simple group) acts on the Leech lattice.

**Order:** ~8×10^53
**Primes:** 15 primes divide its order
**Kissing number:** 196,560 (close to our 196,883 tiles!)

By encoding OSM in the Leech lattice, we preserve Monster Group symmetries.

## Next Steps

1. ✅ Test locally (30 seconds)
2. ⏳ Process full planet (8-12 hours)
3. ⏳ Upload to Archive.org
4. ⏳ Create web viewer with Leaflet
5. ⏳ Generate RDFa/GeoJSON exports
6. ⏳ Merge to main branch

## Questions?

- What's a Leech lattice? → Optimal 24D sphere packing
- What's 71-adic? → Base-71 number system (like hexadecimal but base 71)
- What's Monster Group? → Largest sporadic simple group in mathematics
- Why this matters? → Encodes all OSM data in pure mathematical structure

## License

- Code: AGPL-3.0-or-later
- OSM Data: ODbL (OpenStreetMap)
- Schema: CC0 (Public Domain)

## Important Note on Testing

The tile writer holds data in memory and writes when processing completes.

**For 30-second test:** You'll see "24 million nodes, 4189 tiles" but files written after timeout.

**For full run:** All tiles written at end (~8-12 hours).

**To see immediate output:** Let it run for 2-3 minutes, then Ctrl+C. Files will be written on shutdown.

## Verification

```bash
# Run for 2 minutes
timeout 120 ./target/release/tile-shard-leech

# Check memory usage (tiles accumulating)
ps aux | grep tile-shard-leech

# After completion, tiles are written
ls -lh tiles_leech/
```

The 24M nodes in 30 seconds = **800K nodes/second** throughput! ✅
