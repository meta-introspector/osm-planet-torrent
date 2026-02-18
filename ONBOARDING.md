# OSM Planet Torrent - Onboarding Guide for Kiro Agents

**Welcome!** This guide helps you set up and use the OSM planet torrent indexer.

## What This Does

Index the 85 GB OSM planet torrent by geographic location. Download only the pieces you need (4 MB each) instead of the entire planet.

**Example**: Ramanujan's journey (8 locations) = ~320 MB instead of 85 GB!

## Quick Start

### 1. Clone the Repository

```bash
cd /home/mdupont/projects
git clone https://github.com/meta-introspector/osm-planet-torrent
cd osm-planet-torrent
git submodule update --init --recursive
```

### 2. Build with Nix

```bash
# Using cargo2nix shell
nix develop /home/mdupont/nix/vendor/rust/cargo2nix -c cargo build --release

# Or just run directly
nix develop /home/mdupont/nix/vendor/rust/cargo2nix -c cargo run ramanujan
```

### 3. Create Your User Profile

Create `userdir/yourname.json`:

```json
{
  "user": "yourname",
  "wikidata_user": "Q12345",
  "osm_user": "your_osm_username",
  "locations": [
    {
      "name": "MyCity",
      "lat": 40.7128,
      "lon": -74.0060,
      "wikidata": "Q60",
      "osm_node": null,
      "radius_miles": 10.0
    }
  ]
}
```

**Fields**:
- `radius_miles`: Default 10 miles if not specified
- `wikidata`: Q entity ID (optional)
- `osm_node`: OSM node ID (optional)

### 4. Run Your Profile

```bash
cargo run yourname
```

**Output**:
- `yourname-location-index.json` - Torrent piece mapping
- `yourname-wikidata.json` - Wikidata query results
- `osm-planet.torrent` - Torrent metadata (437 KB)

## Understanding the Output

### Location Index

```json
{
  "user": "yourname",
  "locations": [
    {
      "name": "MyCity",
      "lat": 40.7128,
      "lon": -74.0060,
      "piece": 8580,
      "shard": 60,
      "radius_pieces": [8580, 8581, 8582, ...]
    }
  ]
}
```

- **piece**: Center piece for this location
- **shard**: Monster prime shard (mod 71)
- **radius_pieces**: All pieces in radius

### Wikidata Results

```json
{
  "user": "yourname",
  "queries": [
    {
      "location": "MyCity",
      "qid": "Q60",
      "data": { ... },
      "linked": ["Q8686", "Q1384", ...]
    }
  ]
}
```

Discovers related entities automatically!

## Finding Wikidata Q IDs

### For Places

1. Search: https://www.wikidata.org/
2. Find your location
3. Copy Q ID from URL: `https://www.wikidata.org/wiki/Q60`

### For People

Example: Ramanujan = Q185493
- Search Wikipedia article
- Click "Wikidata item" in left sidebar

## Advanced Usage

### Custom Radius Per Location

```json
{
  "name": "Downtown",
  "lat": 40.7128,
  "lon": -74.0060,
  "radius_miles": 5.0
},
{
  "name": "Suburbs",
  "lat": 40.8000,
  "lon": -74.0000,
  "radius_miles": 20.0
}
```

### Selective Download

Once you have the index, use aria2c or transmission:

```bash
# Download specific pieces
aria2c --select-file=8580,8581,8582 osm-planet.torrent
```

## Integration with Kiro

### Share Your Locations

```bash
# Copy to Zone 42 UUCP spool
cp userdir/yourname.json /mnt/data1/zones/42/uucp/spool/
```

### View on Map

After GitHub Actions runs:
https://meta-introspector.github.io/osm-planet-torrent/

## Troubleshooting

### Build Errors

```bash
# Clean and rebuild
cargo clean
nix develop /home/mdupont/nix/vendor/rust/cargo2nix -c cargo build --release
```

### Missing Dependencies

```bash
# Install OpenSSL
sudo apt-get install pkg-config libssl-dev
```

### Wikidata Query Timeout

Queries are rate-limited. Wait 1 minute between runs.

## Examples

### Example 1: Your Hometown

```json
{
  "user": "hometown",
  "locations": [
    {
      "name": "Home",
      "lat": 39.0473,
      "lon": -95.6752,
      "radius_miles": 15.0
    }
  ]
}
```

### Example 2: Travel Route

```json
{
  "user": "roadtrip",
  "locations": [
    {"name": "Start", "lat": 40.7128, "lon": -74.0060, "radius_miles": 5.0},
    {"name": "Stop1", "lat": 41.8781, "lon": -87.6298, "radius_miles": 5.0},
    {"name": "End", "lat": 37.7749, "lon": -122.4194, "radius_miles": 5.0}
  ]
}
```

### Example 3: Research Project

```json
{
  "user": "temples",
  "wikidata_user": "Q185493",
  "locations": [
    {"name": "Temple1", "lat": 10.9617, "lon": 79.3881, "wikidata": "Q2744680"},
    {"name": "Temple2", "lat": 11.2189, "lon": 78.1677, "wikidata": null}
  ]
}
```

## Monster Group Sharding

Pieces are sharded using the largest Monster prime (71):
- Enables distributed storage across 71 shards
- Each shard can be independently cached
- Follows Monster Group structure (15 primes)

## Contributing

1. Fork the repo
2. Add your locations to `userdir/`
3. Test locally
4. Submit PR (userdir is gitignored, only share if you want)

## Support

- GitHub Issues: https://github.com/meta-introspector/osm-planet-torrent/issues
- Zone 42 UUCP: `/mnt/data1/zones/42/uucp/spool/`
- Kiro Chat: Ask other agents!

## Next Steps

1. Create your user profile
2. Run the indexer
3. View results on the map
4. Download only what you need!

∴ From 85 GB to megabytes! 🌍🎯

---

**Happy mapping!** 🗺️✨
