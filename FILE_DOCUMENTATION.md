# OSM Planet Torrent - File Documentation

## Project Structure

### Core Configuration
- **Cargo.toml** - Rust dependencies (modified)
- **Cargo.lock** - Dependency lock (modified)
- **flake.nix** - Nix build system (modified)
- **flake.lock** - Nix lock (modified)
- **README.md** - Main documentation (modified)

### Monster Convention (NEW)
- **.monster/** - Monster Group integration
  - **symmetries.json** - [71,59,47] → [17,23,59] mapping
  - Derived: port 17206, l71s59z47.monster.local
- **public_html/** - Web interface (created by monster-init)

### Documentation (NEW)
- **BREAKTHROUGH.md** - Key discoveries
- **BROWSER_VIEWER.md** - Browser interface docs
- **HIERARCHICAL_SHARDING.md** - Sharding strategy
- **HUGGINGFACE_STRATEGY.md** - HuggingFace integration
- **MONSTER_INTEGRATION.md** - Monster Group compression
- **NODE_ID_STRATEGY.md** - Node ID calculation
- **QUALITY_MANAGEMENT.md** - Quality control
- **SOLANA_TILE_SERVICE.md** - Solana integration
- **SPATIAL_INDEX_STRATEGY.md** - Spatial indexing
- **TILE_SHARDING_PLAN.md** - Tile sharding

### Source Code (src/)

#### Main Binaries (src/bin/)
- **bbs_map.rs** - BBS door game mapping
- **binary_search_pieces.rs** - Binary search for pieces
- **build_spatial_index.rs** - Build spatial index
- **decode_zlib.rs** - Decompress zlib blocks
- **extract_kumbakonam.rs** - Extract Kumbakonam area
- **fetch_piece.rs** - Download specific piece
- **geo_shard.rs** - Geographic sharding
- **geo_split.rs** - Split by geography
- **index_planet.rs** - Index planet file
- **modular_bucket.rs** - Modular bucketing
- **monster_shard.rs** - Monster Group sharding
- **node_to_piece.rs** - Calculate piece from node ID
- **parse_dense.rs** - Parse DenseNodes
- **parse_one_block.rs** - Parse single block
- **parse_pieces.rs** - Parse multiple pieces
- **query_index.rs** - Query spatial index
- **query_shards.rs** - Query shards
- **ramanujan_shard.rs** - Ramanujan-based sharding
- **read_one_block.rs** - Read single block
- **reconstruct_pbf.rs** - Reconstruct PBF file
- **repack_planet.rs** - Repack planet data
- **reshard_planet.rs** - Reshard planet
- **reshard_to_ramanujan.rs** - Reshard to Ramanujan tiles
- **shard_index.rs** - Shard indexing
- **stream_and_index.rs** - Stream and index
- **temporal_strata.rs** - Temporal stratification
- **test_modular.rs** - Test modular arithmetic
- **tile_parquet.rs** - Tile to Parquet
- **tile_parquet_leech.rs** - Leech lattice tiles
- **tile_shard.rs** - Tile sharding (modified)
- **tile_shard_enhanced.rs** - Enhanced tile sharding
- **tile_shard_leech.rs** - Leech lattice sharding (modified)

#### Library Code (src/)
- **main.rs** - Main entry point (modified)
- **lib.rs** - Library exports
- **download.rs** - Download logic (modified)
- **userdir.rs** - User directory handling (modified)
- **chunk_writer.rs** - Chunk writing
- **piece_download.rs** - Piece download
- **piece_index.rs** - Piece indexing
- **print_storage.rs** - Storage printing
- **stream.rs** - Streaming logic

### Data Files

#### Ramanujan Data
- **ramanujan-wikidata.json** - Wikidata entities (modified)
- **ramanujan-wikidata-crawl.json** - Full crawl (modified)
- **ramanujan-monster-projection.json** - Monster projection
- **ramanujan-location-index.json** - Location index
- **userdir/ramanujan.json** - User directory data (modified)

#### Spatial Indices
- **spatial_index.json** - Basic spatial index
- **complete_spatial_index.jsonl** - Complete index
- **complete_spatial_index_v2.jsonl** - Version 2

#### Kumbakonam Extraction
- **kumbakonam-query.nix** - Nix query
- **kumbakonam_area.jsonl** - Area data
- **kumbakonam_extract.log** - Extraction log
- **kumbakonam_merged.pbf** - Merged PBF

### Shards and Tiles

#### Directories
- **shards/** - 71×59 Monster shards (3.1M files)
- **tiles/** - Generated tiles (520K files)
- **ramanujan_tiles/** - Ramanujan-specific tiles (40K files)
- **test_tiles/** - Test tiles
- **monster_shards/** - Monster Group shards (20K files)
- **geo_shards/** - Geographic shards (12K files)
- **chunks/** - Downloaded chunks (282K files)
- **index/** - Piece index (901K files)

### MiniZinc Optimization
- **hierarchical_shard.mzn** - Hierarchical sharding
- **modular_shard.mzn** - Modular sharding
- **monster_packing.mzn** - Monster packing
- **optimize_tiles.mzn** - Tile optimization

### Scripts
- **build-parquet.sh** - Build Parquet files
- **build_full_index.sh** - Build full index
- **create-parquet.sh** - Create Parquet
- **create-parquet-writer.sh** - Create writer
- **download_direct.sh** - Direct download
- **download_full_planet.sh** - Full planet download
- **query_tiles.py** - Query tiles (Python)
- **rebuild-leech.sh** - Rebuild Leech lattice
- **run-bbs-map.sh** - Run BBS map
- **test-parse.sh** - Test parsing
- **test_reshard.py** - Test resharding (Python)

### Tile Service
- **tile-service/** - Tile serving infrastructure
- **osm-tile-service.nix** - Nix service definition

### Web Interface
- **docs/index.html** - Main web interface (modified)
- **docs/monster-osm-browser.html** - Monster browser
- **docs/monster.html** - Monster visualization
- **docs/data** - Data symlink
- **docs/tiles_json** - Tiles JSON
- **docs/tile-stats.json** - Tile statistics
- **monster-osm-browser.html** - Browser (root)

### Logs and Reports
- **buildlog.txt** - Build log
- **fetch.log** - Fetch log
- **fetch_background.log** - Background fetch (763K)
- **filter.log** - Filter log
- **geo_split.log** - Geo split log
- **index_build.log** - Index build log
- **index_planet.log** - Planet index log
- **log.txt** - General log
- **perf_report.txt** - Performance report
- **piece_download.log** - Piece download log
- **planet_download.log** - Planet download log
- **planet_download_output.log** - Download output
- **report.txt** - Main report
- **reshard_parallel.log** - Parallel reshard log
- **reshard_progress.log** - Reshard progress
- **run.log** - Run log
- **stream_index.log** - Stream index log

### Reconstructed Pieces
- **piece_0000000_reconstructed.pbf** - Piece 0 (10.9 MB)
- **piece_0000001_reconstructed.pbf** - Piece 1 (4.2 MB)
- **piece_0000002_reconstructed.pbf** - Piece 2 (4.1 MB)
- **piece_0000003_reconstructed.pbf** - Piece 3 (3.9 MB)
- **piece_0000004_reconstructed.pbf** - Piece 4 (4.2 MB)
- **piece_0021762_reconstructed.pbf** - Piece 21762 (461 KB)
- **piece_9055.pbf** - Piece 9055 (4.2 MB)
- **piece_*_block_0_decompressed.bin** - Decompressed blocks

### Test Files
- **test_block** - Test binary (553 KB)
- **test_block.rs** - Test source
- **parse_piece.rs** - Parse piece test
- **tests/** - Test directory

### Vendor
- **vendor/rqbit** - Torrent client (submodule, modified)

### Admin
- **admin/** - Admin scripts

### Miscellaneous
- **recover.txt** - Recovery notes
- **recover2.txt** - Recovery notes v2
- **requeue.txt** - Requeue data (29 KB)
- **thread.txt** - Thread info (16 KB)
- **wd.txt** - Working directory notes
- **osm-planet.torrent** - Torrent file (436 KB)
- **planet-latest.osm.pbf.torrent** - Latest torrent (436 KB)

## Statistics

- **Total Rust binaries**: 30+
- **Total shards**: 3.1M files
- **Total tiles**: 520K files
- **Total chunks**: 282K files
- **Reconstructed pieces**: 7 (27 MB total)
- **Documentation files**: 10 MD files
- **MiniZinc models**: 4 files
- **Scripts**: 10+ shell/Python scripts

## Monster Integration

**Symmetries**: [71,59,47] → [17,23,59]
**Port**: 17206
**Hostname**: l71s59z47.monster.local
**Invariants**: geographic, torrent, Monster-Group, OSM

## Next Steps

1. Add untracked files to git
2. Commit Monster integration
3. Update documentation
4. Deploy tile service on port 17206
