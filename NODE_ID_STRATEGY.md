# Direct Node ID Lookup Strategy

## Key Insight
OSM planet file is sorted by **node ID**, not geography!

If we know the node IDs we want, we can:
1. Calculate which piece contains that node ID
2. Download only that specific piece
3. Extract the node data directly

## Example: Kumbakonam Area

Node: Noor Nagar (2824755486)
- https://www.openstreetmap.org/node/2824755486
- Version #2
- Delhi place tag hierarchy reclassification

## Calculation

**OSM Planet Stats:**
- Total pieces: 21,763
- Piece size: 4 MB (4,194,304 bytes)
- Total size: 85 GB
- Estimated total nodes: ~9 billion

**Node ID → Piece ID:**
```
nodes_per_piece ≈ 9,000,000,000 / 21,763 ≈ 413,500 nodes/piece

piece_id = node_id / nodes_per_piece
```

**For Node 2824755486:**
```
piece_id ≈ 2,824,755,486 / 413,500 ≈ 6,831
```

## Strategy

1. **Get target node IDs** from OSM API or Overpass
   - Query: "All nodes in Kumbakonam with wikidata tags"
   - Returns list of node IDs

2. **Calculate piece IDs** for each node
   - Use formula: `piece_id = node_id / nodes_per_piece`
   - Adjust based on actual distribution from samples

3. **Download specific pieces** (10-50 pieces = 40-200 MB)
   - Much more efficient than sampling 307 pieces!

4. **Extract nodes** from those pieces
   - Parse DenseNodes
   - Filter by target node IDs
   - Extract wikidata Q IDs

## Next Steps

1. Query Overpass API for Kumbakonam nodes
2. Refine nodes_per_piece estimate from existing samples
3. Calculate target piece IDs
4. Download and parse specific pieces
