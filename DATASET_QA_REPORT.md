# Dataset QA Report - 6 Sigma + CFT

Generated: 2026-02-22

## Phase Transitions

1. chunks → index (Liquid → Solid)
2. index → shards (Solid → Crystal)
3. shards → tiles (Crystal → Plasma)

---

## Dataset: chunks

- **Total**: 4418
- **Sample**: 67 (√4418 + 1)
- **Phase**: Liquid

### Test Cases (first 10)

```
chunks/piece_0000000_offset_0002027003.bin
chunks/piece_0000003_offset_0016662528.bin
chunks/piece_0000001_offset_0007072575.bin
chunks/piece_0000001_offset_0005631879.bin
chunks/piece_0000001_offset_0008306688.bin
chunks/piece_0000000_offset_0001503058.bin
chunks/piece_0000002_offset_0012304384.bin
chunks/piece_0000000_offset_0002605056.bin
chunks/piece_0000000_offset_0004007810.bin
chunks/piece_0000000_offset_0002976448.bin
```

### Structure

- README: ✓
- FILE_LIST: ✓

---

## Dataset: index

- **Total**: 21792
- **Sample**: 148 (√21792 + 1)
- **Phase**: Solid

### Test Cases (first 10)

```
index/FILE_LIST.txt
index/.gitattributes
index/README.md
index/shard_0014354.json
index/shard_0016331.json
index/shard_0018639.json
index/shard_0010676.json
index/shard_0019587.json
index/shard_0009594.json
index/shard_0012997.json
```

### Structure

- README: ✓
- FILE_LIST: ✓

---

## Dataset: shards

- **Total**: 90270
- **Sample**: 301 (√90270 + 1)
- **Phase**: Crystal

### Test Cases (first 10)

```
shards/README.md
shards/.gitattributes
shards/FILE_LIST.txt
shards/b_1_36_12.csv
shards/b_2_12_14.csv
shards/b_3_24_30.csv
shards/b_4_21_08.csv
shards/b_10_13_30.csv
shards/b_54_29_04.csv
shards/b_69_30_04.csv
```

### Structure

- README: ✓
- FILE_LIST: ✓

---

## Dataset: tiles

- **Total**: 986393
- **Sample**: 994 (√986393 + 1)
- **Phase**: Plasma

### Test Cases (first 10)

```
tiles/tile_137_67/nodes_57.csv
tiles/tile_137_67/nodes_07.csv
tiles/tile_137_67/nodes_27.csv
tiles/tile_137_67/nodes_55.csv
tiles/tile_137_67/nodes_64.csv
tiles/tile_137_67/nodes_61.csv
tiles/tile_137_67/nodes_19.csv
tiles/tile_137_67/nodes_40.csv
tiles/tile_137_67/nodes_48.csv
tiles/tile_137_67/nodes_10.csv
```

### Structure

- README: ✓
- FILE_LIST: ✓

---

## Summary

**Status**: ✓ READY

