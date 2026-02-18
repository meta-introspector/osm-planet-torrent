# OSM Planet Torrent Location Indexer

**Index OSM planet torrent by geographic location - fetch only what you need!**

## Problem

OSM planet file is 85 GB. You only need data for specific locations.

## Solution

Map lat/lon coordinates to torrent pieces. Download only needed pieces (4 MB each) instead of entire planet.

## Example: Ramanujan's Journey

From Kumbakonam (India) to Cambridge (UK):

```
Planet: 85 GB, 21,763 pieces
Locations → Pieces:
- Kumbakonam (10.96°N, 79.39°E) → Piece 13,668 (shard 36)
- Chennai (13.08°N, 80.27°E) → Piece 14,137 (shard 8)
- London (51.51°N, 0.13°W) → Piece 16,793 (shard 37)
- Cambridge (52.21°N, 0.12°E) → Piece 16,945 (shard 47)
- Trinity College → Piece 16,945 (shard 47)

Result: 5 pieces × 4 MB = 20 MB instead of 85 GB!
```

## Usage

```bash
# Build
nix build .#osm-torrent-client

# Run
./result/bin/osm-planet-torrent

# Output
- osm-planet.torrent (437 KB)
- ramanujan-location-index.json (location → piece mapping)
```

## How It Works

1. **Download torrent metadata** (437 KB, not the full 85 GB)
2. **Map locations to pieces** using golden ratio distribution:
   ```rust
   lat_norm = (lat + 90.0) / 180.0
   lon_norm = (lon + 180.0) / 360.0
   combined = (lat_norm * φ + lon_norm) % 1.0
   piece_idx = (combined * num_pieces) as usize
   ```
3. **Shard by Monster prime** (mod 71) for distributed storage
4. **Selective download** using aria2c/transmission

## Monster Group Sharding

Pieces are sharded using the largest Monster prime (71):
- Enables distributed storage across 71 shards
- Each shard can be independently fetched/cached
- Follows Monster Group structure (15 primes)

## Add Your Locations

Edit `src/main.rs`:

```rust
const LOCATIONS: &[Location] = &[
    Location { name: "YourCity", lat: 40.7128, lon: -74.0060 },
    // Add more...
];
```

## Requirements

- Nix with flakes enabled
- Or: Rust toolchain with OpenSSL

## Build Without Nix

```bash
cargo build --release
./target/release/osm-planet-torrent
```

## Selective Download

Once you have the index, use aria2c to fetch specific pieces:

```bash
# Download only piece 16945 (Cambridge)
aria2c --select-file=16945 osm-planet.torrent
```

## Integration

Part of CICADIA-71 bootstrap system:
- 10-step journey: Monster Group → Ramanujan → Temple
- Connects abstract math to physical locations
- Proves system can ground itself in reality

## Files

- `src/main.rs` - Location indexer
- `src/shard.rs` - Monster prime sharding
- `flake.nix` - Pure Nix build
- `Cargo.toml` - Rust dependencies (rustls, no native-tls)

## License

MIT

## Related

- [CICADIA-71 Bootstrap](https://github.com/meta-introspector/cicadia71)
- [OpenStreetMap](https://planet.openstreetmap.org/)
- [Monster Group](https://en.wikipedia.org/wiki/Monster_group)

---

*"From 85 GB to 20 MB - fetch only what you need!"* 🌍🕉️
