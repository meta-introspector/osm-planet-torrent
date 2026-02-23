# Monster OSM Quest - Workflows & Testing

## 🎭 Complete Integration

All tools now use **Nix** for reproducible builds and testing.

## Quick Commands

```bash
# Enter Nix development shell
nix develop --impure

# Test zkperf extraction (16ms)
./test-zkperf-nix.sh 13668 5

# Test sparse torrent reading
./test-sparse-nix.sh 13668 10

# Build all binaries
nix develop --impure -c cargo build --release --bins

# Run with nix
nix run .#zkperf-dense -- --help
```

## GitHub Actions

### 1. Sparse Torrent Test
**File**: `.github/workflows/sparse-torrent-test.yml`

Tests sparse piece extraction with ZK witnesses.

**Trigger**:
```bash
gh workflow run sparse-torrent-test.yml -f piece=13668 -f limit=10
```

### 2. 24 Walkers CI
**File**: `.github/workflows/walkers-ci.yml`

Runs walker simulation every 6 hours.

**Trigger**:
```bash
gh workflow run walkers-ci.yml
```

## Test Results

**Piece 13668** (Kumbakonam):
- ⏱️ 16ms extraction
- 📊 5 nodes
- 🔐 SHA256: `b8a96e617fefc14f...`
- 📦 5 zlib blocks
- 💾 4.2MB data

## Documentation

- `NIX_INTEGRATION.md` - Complete Nix setup
- `CI_TESTING.md` - CI/CD guide
- `README_ASCIINEMA.md` - Asciinema movies

## Asciinema Movies

- 🎮 Ultima Quest: https://asciinema.org/a/f8QZVlQ5wDp3DdnA
- 🎵 Musical Walk: https://asciinema.org/a/kigZKXMZd9hPcISE

---

🎭 All systems operational!
