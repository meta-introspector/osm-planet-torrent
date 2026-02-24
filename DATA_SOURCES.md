# Monster OSM Quest - Data Sources

## Archive.org Collections

### 1. OSM Planet Data
**Collection**: `osm-planet-monster-shards`
**URL**: https://archive.org/details/osm-planet-monster-shards

**Files**:
- `planet-latest.osm.pbf` (86GB) - Full OSM planet
- `planet-latest.osm.pbf.torrent` - BitTorrent metadata
- `piece-index.json` - Piece to shard mapping (71 shards)
- `shard-manifest.json` - Shard statistics

### 2. LMFDB Mathematical Data
**Collection**: `lmfdb-monster-71-shards`
**URL**: https://archive.org/details/lmfdb-monster-71-shards

**Files**:
- `lmfdb_71_shards.json` (49 shards, 75 functions)
- `lmfdb_math_functions.json` - Function metadata
- `lmfdb_knowledge_base.pl` - Prolog knowledge base

### 3. Ramanujan Locations
**Collection**: `ramanujan-osm-locations`
**URL**: https://archive.org/details/ramanujan-osm-locations

**Files**:
- `ramanujan-location-index.json` - 8 biographical locations
- `kumbakonam-piece-13668.geojson` - Birth location nodes
- `cambridge-piece-16945.geojson` - Trinity College nodes
- `london-piece-16793.geojson` - Hardy collaboration

### 4. Musical Periodic Table
**Collection**: `monster-musical-periodic-table`
**URL**: https://archive.org/details/monster-musical-periodic-table

**Files**:
- `MUSICAL_PERIODIC_TABLE.md` - Complete documentation
- `monster-primes-frequencies.json` - 15 primes with frequencies
- `harmonic-mappings.json` - Semantic emoji annotations

## Hugging Face Datasets

### 1. Monster OSM Shards
**Dataset**: `meta-introspector/monster-osm-shards`
**URL**: https://huggingface.co/datasets/meta-introspector/monster-osm-shards

**Structure**:
```
monster-osm-shards/
├── README.md
├── shards/
│   ├── shard-00.parquet (Pacific Ocean)
│   ├── shard-17.parquet (Giza - Cusp)
│   ├── shard-23.parquet (Silicon Valley - Consciousness)
│   ├── shard-59.parquet (Ramanujan Temple - Memory)
│   └── ... (71 total)
├── metadata.json
└── piece-to-shard-mapping.json
```

### 2. LMFDB Monster Integration
**Dataset**: `meta-introspector/lmfdb-monster-71`
**URL**: https://huggingface.co/datasets/meta-introspector/lmfdb-monster-71

**Structure**:
```
lmfdb-monster-71/
├── README.md
├── lmfdb_71_shards.json
├── elliptic_curves/
│   └── curves-mod-71.parquet
├── modular_forms/
│   └── forms-mod-71.parquet
├── l_functions/
│   └── l-functions-mod-71.parquet
└── number_fields/
    └── fields-mod-71.parquet
```

### 3. Ramanujan Mathematical Journey
**Dataset**: `meta-introspector/ramanujan-osm-journey`
**URL**: https://huggingface.co/datasets/meta-introspector/ramanujan-osm-journey

**Structure**:
```
ramanujan-osm-journey/
├── README.md
├── locations.json (8 biographical locations)
├── fractran-encoding.json (FRACTRAN states)
├── geojson/
│   ├── kumbakonam.geojson
│   ├── chennai.geojson
│   ├── london.geojson
│   └── cambridge.geojson
└── witnesses/
    └── zk-witnesses.json (cryptographic proofs)
```

### 4. 24 Walker Simulation Data
**Dataset**: `meta-introspector/ramanujan-24-walkers`
**URL**: https://huggingface.co/datasets/meta-introspector/ramanujan-24-walkers

**Structure**:
```
ramanujan-24-walkers/
├── README.md
├── walker-paths.parquet (240 steps × 24 agents)
├── discoveries.json (LMFDB nodes found)
├── shard-visits.json (71 shards coverage)
└── statistics.json (distance, time, coverage)
```

## Download Scripts

### Archive.org
```bash
# Download LMFDB data
ia download lmfdb-monster-71-shards

# Download Ramanujan locations
ia download ramanujan-osm-locations

# Download Musical Periodic Table
ia download monster-musical-periodic-table
```

### Hugging Face
```bash
# Install datasets library
pip install datasets huggingface_hub

# Download Monster OSM shards
from datasets import load_dataset
ds = load_dataset("meta-introspector/monster-osm-shards")

# Download LMFDB integration
ds = load_dataset("meta-introspector/lmfdb-monster-71")

# Download Ramanujan journey
ds = load_dataset("meta-introspector/ramanujan-osm-journey")

# Download walker simulation
ds = load_dataset("meta-introspector/ramanujan-24-walkers")
```

## CI/CD Integration

Update workflows to download from Archive.org/HuggingFace:

```yaml
- name: Download test data
  run: |
    # Install ia tool
    pip install internetarchive
    
    # Download LMFDB data
    ia download lmfdb-monster-71-shards lmfdb_71_shards.json
    mkdir -p spool
    mv lmfdb-monster-71-shards/lmfdb_71_shards.json spool/
    
    # Or use HuggingFace
    pip install datasets
    python3 << 'EOF'
    from datasets import load_dataset
    ds = load_dataset("meta-introspector/lmfdb-monster-71", split="train")
    ds.to_json("spool/lmfdb_71_shards.json")
    EOF
```

## Data Sizes

| Dataset | Size | Format | Location |
|---------|------|--------|----------|
| OSM Planet | 86GB | PBF | Archive.org |
| LMFDB Shards | 2.4MB | JSON | Both |
| Ramanujan Locations | 156KB | GeoJSON | Both |
| Musical Table | 48KB | Markdown | Both |
| Walker Paths | 1.2MB | Parquet | HuggingFace |
| Monster Shards | 12GB | Parquet | HuggingFace |

## Licenses

- **OSM Data**: ODbL (Open Database License)
- **LMFDB**: CC BY-SA 4.0
- **Code**: MIT
- **Documentation**: CC BY 4.0

## Checksums

All files include SHA256 checksums in `checksums.txt`:
```
b8a96e617fefc14f1ecbd62d238812d06ff9676f5ddec47edb4316a8fddb3f0c  lmfdb_71_shards.json
...
```

---

🎭 Monster OSM Quest - All data publicly archived
