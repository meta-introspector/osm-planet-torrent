# Monster OSM Quest - CI/CD & Sparse Torrent Testing

## GitHub Actions Workflows

### 🌐 Sparse Torrent + ZKPerf Test
**File**: `.github/workflows/sparse-torrent-test.yml`

Tests sparse torrent reading with zkperf_dense extractor.

**Features**:
- Downloads only specific pieces via aria2
- Extracts dense nodes with ZK witnesses
- FRACTRAN encoding of piece data
- Uploads artifacts for verification

**Trigger**:
```bash
# Manual dispatch
gh workflow run sparse-torrent-test.yml -f piece=13668 -f limit=10

# Or push to main
git push origin main
```

### 🧮 24 Ramanujan Walkers CI
**File**: `.github/workflows/walkers-ci.yml`

Runs walker simulation with LMFDB discovery.

**Features**:
- 24 walker agents across 71 shards
- LMFDB/OEIS/Wikidata node discovery
- Math database projection
- Scheduled runs every 6 hours

**Trigger**:
```bash
# Manual dispatch
gh workflow run walkers-ci.yml

# Automatic on schedule
# Runs at 00:00, 06:00, 12:00, 18:00 UTC
```

## Local Testing

### Sparse Torrent Test
```bash
# Test with default piece (13668 - Kumbakonam)
./test-sparse-torrent.sh

# Test specific piece
./test-sparse-torrent.sh 16945 20  # Cambridge, 20 nodes

# Test sacred shards
./test-sparse-torrent.sh 17 10   # Cusp
./test-sparse-torrent.sh 23 10   # Consciousness
./test-sparse-torrent.sh 59 10   # Memory
```

**Requirements**:
- `aria2c` - Torrent client
- `jq` - JSON processor
- Rust toolchain

**Install**:
```bash
sudo apt-get install aria2 jq
```

### Direct zkperf_dense
```bash
# Build
cargo build --release --bin zkperf_dense

# Extract from local planet
./target/release/zkperf_dense \
  --input /mnt/data1/osm-planet/planet-latest.osm.pbf \
  --output nodes.geojson \
  --piece 13668 \
  --limit 10

# Check results
jq '.features | length' nodes.geojson
cat nodes.geojson.witness.json
```

## Torrent Sparse Reading

### How It Works

1. **Piece Calculation**
   - OSM planet: ~86GB = ~5,120 pieces (16MB each)
   - Piece → Shard: `(piece * 71) / 86000`
   - Offset: `piece * 16777216`

2. **Sparse Download**
   - aria2c downloads only requested pieces
   - DHT/PEX for peer discovery
   - No seeding (--seed-time=0)

3. **Extraction**
   - zkperf_dense reads piece directly
   - Decompresses zlib blocks
   - Parses dense nodes
   - Generates ZK witness

### Sacred Shard Pieces

| Shard | Location | Piece Range | Command |
|-------|----------|-------------|---------|
| 17 | Giza (Cusp) | 20,600-20,700 | `./test-sparse-torrent.sh 20650` |
| 23 | Silicon Valley | 27,800-27,900 | `./test-sparse-torrent.sh 27850` |
| 59 | Ramanujan Temple | 71,400-71,500 | `./test-sparse-torrent.sh 71450` |

### Ramanujan Locations

| Location | Piece | Shard | Command |
|----------|-------|-------|---------|
| Kumbakonam Birth | 13668 | 36 | `./test-sparse-torrent.sh 13668` |
| Namagiri Temple | 13645 | 13 | `./test-sparse-torrent.sh 13645` |
| Chennai College | 14137 | 8 | `./test-sparse-torrent.sh 14137` |
| London Hardy | 16793 | 37 | `./test-sparse-torrent.sh 16793` |
| Cambridge Trinity | 16945 | 47 | `./test-sparse-torrent.sh 16945` |

## Integration with Existing Tools

### FRACTRAN Encoding
```bash
cargo build --release --bin fractran_osm
./target/release/fractran_osm --piece 13668
```

### 24 Walkers
```bash
cargo build --release --bin ramanujan_24_walkers
./target/release/ramanujan_24_walkers
```

### LMFDB Discovery
```bash
cargo build --release --bin walkers_with_lmfdb
./target/release/walkers_with_lmfdb \
  --lmfdb-path /mnt/data1/spool/experiments_monster/lmfdb_71_shards.json
```

## Artifacts

All workflows upload artifacts:
- `sparse-extraction-piece-{N}` - Extracted nodes + witnesses
- `walker-simulation-{N}` - Walker paths + discoveries

**Download**:
```bash
gh run download {run-id}
```

## Performance

| Operation | Time | Output |
|-----------|------|--------|
| Sparse download (1 piece) | ~30s | 16MB |
| zkperf_dense extract | ~16ms | 10 nodes |
| FRACTRAN encode | ~4ms | 1 state |
| 24 walkers | ~9ms | 240 steps |
| LMFDB discovery | ~2ms | 75 nodes |

## Monitoring

Check workflow status:
```bash
gh workflow list
gh run list --workflow=sparse-torrent-test.yml
gh run watch
```

---

🎭 Monster OSM Quest - Sparse torrent reading with ZK witnesses
