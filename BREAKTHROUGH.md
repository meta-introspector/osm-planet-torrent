# OSM Planet Torrent PBF Recovery - BREAKTHROUGH

**Date**: 2026-02-18 19:07

## What We Achieved

Successfully recovered and parsed OpenStreetMap data from torrent chunks WITHOUT downloading the full 85 GB planet file!

### Results

**Piece 1 Block 0**: 47 Wikidata entities extracted
- Honolulu (Q18094) at 21.3045°, -157.8557°
- Preston (Q184090) at 53.7593°, -2.6993°
- Hilo (Q216258) at 19.7074°, -155.0816°
- Koror (Q527748) at 7.3433°, 134.4767°

**Piece 4 Block 0**: 47 more entities

## Architecture

### Phase 1: Index Building ✅
- Downloaded torrent metadata (437 KB)
- Built index of 21,763 pieces
- Sharded by Monster Group mod 2,081,933 (71×59×497)
- Saved 86 MB index in `index/` directory

### Phase 2: Selective Download ✅
- Custom `PrintStorage` intercepts torrent writes
- Saves chunks without 80GB sparse file
- Downloaded 32 MB from 6 pieces (3,347 chunks)

### Phase 3: PBF Recovery ✅
1. **Reconstruct pieces** from chunks (concatenate by offset)
2. **Find zlib blocks** (signature `0x78 0x9c` or `0x78 0xda`)
3. **Decompress** using flate2::ZlibDecoder
4. **Parse PrimitiveBlock** protobuf with prost
5. **Decode DenseNodes** with delta encoding:
   - `id`, `lat`, `lon` are delta-encoded (accumulate deltas)
   - `keys_vals` is flattened tag list (key, val, key, val, ..., 0)
   - Convert to degrees: `1e-9 × (offset + granularity × encoded)`

## Binary Search Strategy (UPDATED)

### Discovery: OSM is sorted by Node ID!

**Key Insight:** OSM planet file is sorted by **node ID**, not geography!

From piece 1 analysis:
- Piece 1 contains nodes 20,933,784 to 21,458,266
- Range: ~524,482 nodes per piece

### Direct Node ID Lookup

**If we know the node IDs we want:**
```rust
piece_id = node_id / 524_482
```

**Example:** Node 2824755486 (Noor Nagar, Delhi)
- https://www.openstreetmap.org/node/2824755486
- Calculated piece: 5,385
- Byte offset: 21,540 MB

### Optimized Strategy

1. **Query Overpass API** for target area:
   ```
   [out:json];
   (
     node["wikidata"](around:10000,10.9617,79.3881);
   );
   out ids;
   ```

2. **Calculate piece IDs** for each node:
   ```bash
   cargo run --bin node-to-piece -- <node_id1> <node_id2> ...
   ```

3. **Download specific pieces** (10-50 pieces = 40-200 MB)
   - Much more efficient than sampling!

4. **Extract and verify** nodes from pieces

### Tools

- `node-to-piece` - Calculate piece ID from node ID
- `fetch-piece` - Download specific piece
- `reconstruct-pbf` - Reassemble chunks
- `decode-zlib` - Decompress blocks
- `parse-dense` - Extract nodes with wikidata tags

## Key Files

- `src/bin/fetch_piece.rs` - Download specific piece with PrintStorage
- `src/bin/reconstruct_pbf.rs` - Reassemble chunks into pieces
- `src/bin/decode_zlib.rs` - Find and decompress zlib blocks
- `src/bin/parse_dense.rs` - Parse DenseNodes and extract entities
- `src/print_storage.rs` - Custom storage that saves chunks

## Next Steps

1. Create `binary_search_pieces.rs` - Sample first/middle/last pieces
2. Extract bounding boxes from each piece
3. Implement spatial binary search
4. Download only pieces containing Kumbakonam region
5. Extract all Wikidata Q IDs from target area
6. Build spatial index mapping piece → bbox

## Monster Group Connection

- Total pieces: 21,763
- Sharding: 71 × 59 × 497 = 2,081,933 (Monster Group primes)
- Each piece: 4 MB (4,194,304 bytes)
- Total: 85 GB planet file
- **Downloaded so far**: 32 MB (0.038%)
- **Extracted**: 94 Wikidata entities from 2 pieces

The 71-layer system enables efficient spatial indexing through Monster Group arithmetic!
