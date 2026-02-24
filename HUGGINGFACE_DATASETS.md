# HuggingFace Dataset Integration

## Datasets Initialized

All large data directories are now independent git repositories ready for HuggingFace:

1. **shards/** - Monster Group shards (71×59 grid, 3.1M files)
2. **tiles/** - OSM tiles (520K files)
3. **chunks/** - Downloaded torrent chunks (282K files)
4. **index/** - Piece index (901K files)
5. **ramanujan_tiles/** - Ramanujan-specific tiles (40K files)
6. **monster_shards/** - Monster Group shards (20K files)
7. **geo_shards/** - Geographic shards (12K files)

Each dataset has:
- ✅ Git repository initialized
- ✅ README.md with Monster symmetries
- ✅ .gitattributes for Git LFS (*.pbf, *.parquet, *.bin)
- ✅ Initial commit

## Archive.org Integration

All datasets are also archived on Archive.org for long-term preservation.

### Archive.org URLs

- https://archive.org/details/osm-planet-shards-monster-group
- https://archive.org/details/osm-planet-tiles-monster-group
- https://archive.org/details/osm-planet-chunks-monster-group
- https://archive.org/details/osm-planet-index-monster-group
- https://archive.org/details/osm-planet-ramanujan_tiles-monster-group
- https://archive.org/details/osm-planet-monster_shards-monster-group
- https://archive.org/details/osm-planet-geo_shards-monster-group

### Upload to Archive.org

```bash
# Configure (first time only)
nix-shell -p python3Packages.internetarchive --run "ia configure"

# Upload all datasets
./upload-archive.sh
```

## HuggingFace URLs

Datasets will be available at:
- https://huggingface.co/datasets/introspector/osm-planet-shards
- https://huggingface.co/datasets/introspector/osm-planet-tiles
- https://huggingface.co/datasets/introspector/osm-planet-chunks
- https://huggingface.co/datasets/introspector/osm-planet-index
- https://huggingface.co/datasets/introspector/osm-planet-ramanujan_tiles
- https://huggingface.co/datasets/introspector/osm-planet-monster_shards
- https://huggingface.co/datasets/introspector/osm-planet-geo_shards

## Usage

```python
from datasets import load_dataset

# Load Monster Group shards
shards = load_dataset("introspector/osm-planet-shards")

# Load tiles
tiles = load_dataset("introspector/osm-planet-tiles")

# Load Ramanujan tiles
ramanujan = load_dataset("introspector/osm-planet-ramanujan_tiles")
```

## Next Steps

### 1. Archive.org (Long-term Preservation)

```bash
# Configure (first time only)
nix-shell -p python3Packages.internetarchive --run "ia configure"

# Upload all datasets to Archive.org
./upload-archive.sh
```

### 2. HuggingFace CLI

```bash
# Login (token saved to ~/.cache/huggingface/stored_tokens)
nix-shell -p python3Packages.huggingface-hub --run "hf auth login"

# Or use the upload script (handles login automatically)
./upload-datasets.sh
```

**Note:** Use `hf` command (not `huggingface-cli`, which is deprecated)

### 3. Create HuggingFace Repositories

**IMPORTANT:** Before pushing, create the repositories on HuggingFace:

Go to https://huggingface.co/new-dataset and create 7 datasets:
- `introspector/osm-planet-shards`
- `introspector/osm-planet-tiles`
- `introspector/osm-planet-chunks`
- `introspector/osm-planet-index`
- `introspector/osm-planet-ramanujan_tiles`
- `introspector/osm-planet-monster_shards`
- `introspector/osm-planet-geo_shards`

Set each to:
- License: **ODbL-1.0** (OpenStreetMap)
- Visibility: **Public**

### 4. Push to HuggingFace

```bash
# Automated upload (after creating repos above)
./upload-datasets.sh
```

Or manually:
```bash
cd shards
git remote add origin https://huggingface.co/datasets/introspector/osm-planet-shards
git push -u origin main

cd ../tiles
git remote add origin https://huggingface.co/datasets/introspector/osm-planet-tiles
git push -u origin main

cd ../chunks
git remote add origin https://huggingface.co/datasets/introspector/osm-planet-chunks
git push -u origin main

cd ../index
git remote add origin https://huggingface.co/datasets/introspector/osm-planet-index
git push -u origin main

cd ../ramanujan_tiles
git remote add origin https://huggingface.co/datasets/introspector/osm-planet-ramanujan_tiles
git push -u origin main

cd ../monster_shards
git remote add origin https://huggingface.co/datasets/introspector/osm-planet-monster_shards
git push -u origin main

cd ../geo_shards
git remote add origin https://huggingface.co/datasets/introspector/osm-planet-geo_shards
git push -u origin main
```

## Monster Symmetries

All datasets share the same Monster Group structure:
- **Input**: [71, 59, 47] (Keter/Binah/Chokmah)
- **Output**: [17, 23, 59] (Cusp/Consciousness/Memory)
- **Invariants**: geographic, torrent, Monster-Group, OSM

## License

All datasets use ODbL (OpenStreetMap Database License).

## Parent Project

https://github.com/introspector/osm-planet-torrent
