# Hierarchical Spatial Sharding with Monster Group Primes

## Monster Group Order
```
|M| = 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 41 × 47 × 59 × 71
```

## 15 Supersingular Primes (Monster Primes)
2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71

## Hierarchical Subdivision Strategy

### Level 0: Binary Subdivision (2^46)
- Finest granularity: 2^46 = 70,368,744,177,664 cells
- Each cell: ~0.000000003° × ~0.000000005° (sub-meter precision)
- Use for point-like objects (single nodes)

### Level 1: Ternary (3^20)
- 3^20 = 3,486,784,401 cells
- Each cell: ~0.00005° × ~0.0001° (~5-10 meters)
- Use for small buildings, POIs

### Level 2: Quinary (5^9)
- 5^9 = 1,953,125 cells
- Each cell: ~0.09° × ~0.18° (~10 km)
- Use for neighborhoods, small towns

### Level 3: Septenary (7^6)
- 7^6 = 117,649 cells
- Each cell: ~1.5° × ~3° (~150 km)
- Use for cities, regions

### Level 4: Base-11 (11^2)
- 11^2 = 121 cells
- Each cell: ~16° × ~33° (~1,800 km)
- Use for countries, large regions

### Level 5: Base-13 (13^3)
- 13^3 = 2,197 cells
- Each cell: ~8° × ~16° (~900 km)
- Use for provinces, states

### Level 6-15: Coarser Grids
- 17, 19, 23, 29, 31, 41, 47, 59, 71
- Use for continents, global queries

## Sharding Algorithm

```rust
struct SpatialShard {
    level: u8,              // Which Monster prime (0-14)
    shard_id: u64,          // Cell ID at this level
    min_lat: f64,
    max_lat: f64,
    min_lon: f64,
    max_lon: f64,
    piece_ids: Vec<u32>,    // OSM pieces in this shard
    object_count: usize,
    wikidata_count: usize,
    is_boundary: bool,      // Crosses shard boundary
}

fn assign_to_shards(piece: &PieceIndex) -> Vec<SpatialShard> {
    let mut shards = Vec::new();
    
    // Calculate size of piece
    let lat_span = piece.max_lat - piece.min_lat;
    let lon_span = piece.max_lon - piece.min_lon;
    let area = lat_span * lon_span;
    
    // Choose appropriate level based on size
    let level = if area < 0.0001 {
        0  // 2^46 - point-like
    } else if area < 0.01 {
        1  // 3^20 - small
    } else if area < 1.0 {
        2  // 5^9 - medium
    } else if area < 10.0 {
        3  // 7^6 - large
    } else {
        4  // 11^2 - very large
    };
    
    // Calculate which cells this piece overlaps
    let cells = calculate_overlapping_cells(piece, level);
    
    for cell_id in cells {
        shards.push(SpatialShard {
            level,
            shard_id: cell_id,
            min_lat: piece.min_lat,
            max_lat: piece.max_lat,
            min_lon: piece.min_lon,
            max_lon: piece.max_lon,
            piece_ids: vec![piece.piece_id],
            object_count: piece.node_count,
            wikidata_count: piece.wikidata_count,
            is_boundary: cells.len() > 1,
        });
    }
    
    shards
}
```

## Binary Subdivision (Level 0: 2^46)

For finest granularity:

```rust
fn binary_shard_id(lat: f64, lon: f64) -> u64 {
    // Normalize to [0, 1]
    let lat_norm = (lat + 90.0) / 180.0;
    let lon_norm = (lon + 180.0) / 360.0;
    
    // Interleave bits (Z-order curve / Morton code)
    let lat_bits = (lat_norm * (1u64 << 23) as f64) as u64;
    let lon_bits = (lon_norm * (1u64 << 23) as f64) as u64;
    
    let mut shard_id = 0u64;
    for i in 0..23 {
        shard_id |= ((lat_bits >> i) & 1) << (2 * i);
        shard_id |= ((lon_bits >> i) & 1) << (2 * i + 1);
    }
    
    shard_id
}
```

## Boundary Handling

Objects crossing shard boundaries are stored as "arrows":

```rust
struct BoundaryArrow {
    from_shard: u64,
    to_shard: u64,
    piece_id: u32,
    crossing_type: CrossingType,
}

enum CrossingType {
    Horizontal,  // Crosses latitude boundary
    Vertical,    // Crosses longitude boundary
    Corner,      // Crosses both
}
```

## Query Strategy

```rust
fn query_location(lat: f64, lon: f64) -> Vec<u32> {
    let mut pieces = Vec::new();
    
    // Start with finest level (2^46)
    let shard_id = binary_shard_id(lat, lon);
    pieces.extend(query_shard(0, shard_id));
    
    // Check boundary arrows
    pieces.extend(query_boundary_arrows(0, shard_id));
    
    // If not enough results, go up hierarchy
    if pieces.len() < 10 {
        let parent_shard = shard_id / 8;  // 2^3 = 8 children per parent
        pieces.extend(query_shard(1, parent_shard));
    }
    
    pieces
}
```

## Storage Format

```
shards/
├── level_0_binary/          # 2^46 shards (sparse, only populated cells)
│   ├── shard_0000000000000000.json
│   ├── shard_0000000000000001.json
│   └── ...
├── level_1_ternary/         # 3^20 shards
├── level_2_quinary/         # 5^9 shards
├── level_3_septenary/       # 7^6 shards
├── level_4_base11/          # 11^2 shards
└── boundary_arrows/         # Cross-boundary objects
    ├── level_0_arrows.json
    ├── level_1_arrows.json
    └── ...
```

## Next Steps

1. Implement binary subdivision (2^46)
2. Assign each piece to appropriate level
3. Detect boundary crossings
4. Create arrow index for boundaries
5. Build hierarchical query system
6. Upload to HuggingFace as dataset
