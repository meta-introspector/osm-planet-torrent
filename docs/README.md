# OSM + Wikidata Interactive Map

**Live Demo:** [GitHub Pages](https://meta-introspector.github.io/osm-planet-torrent/)

## Features

### 🗺️ Interactive Map
- Click anywhere to query OSM data for that location
- 35 Wikidata locations pre-loaded (Ramanujan's journey)
- Leaflet.js with OpenStreetMap tiles

### 📦 Ramanujan Tile System
- **71 × 59 × 47 = 196,883 tiles**
- Hierarchical sharding using Ramanujan primes
- Each tile ~4 KB (compressed)
- Total: 86 GB planet → 196,883 queryable tiles

### 🔍 Query System
- Enter coordinates or click map
- Calculates Ramanujan tile: `(lat%71, lon%59, level%47)`
- Direct download from Archive.org
- No server needed - pure static site

### 📍 Wikidata Integration
- 35 locations from Ramanujan's life
- Royal Society, Trinity Hall, Cambridge
- Kumbakonam, Chennai, British Raj
- Each with Wikidata ID and tile coordinates

## Architecture

```
User clicks (lat, lon)
  ↓
Calculate tile: 
  tile_lat = ((lat + 90) * 100) % 71
  tile_lon = ((lon + 180) * 100) % 59
  tile_level = admin_level % 47
  ↓
Download from Archive.org:
  tile_LL_OO_HH.csv
  ↓
Display OSM nodes for that location
```

## Data Sources

- **OSM Planet:** 86 GB PBF file
- **Wikidata:** SPARQL queries for Ramanujan entities
- **Archive.org:** [osm-ramanujan-tiles-sample](https://archive.org/details/osm-ramanujan-tiles-sample)

## Ramanujan Primes

Using the **15 Monster Group primes**:
```
2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71
```

And **20 Ramanujan primes**:
```
2, 11, 17, 29, 41, 47, 59, 67, 71, 97, 101, 107, 127, 149, 151, 167, 179, 181, 227, 229
```

**71** is both:
- The 20th prime number
- The largest Monster prime (Omega prime)
- The maximum depth for graph traversal

## Usage

### Query by Coordinates
```javascript
// Enter lat/lon in form
lat: 10.9617
lon: 79.3881
→ Downloads tile_14_38_00.csv
```

### Click Map
```javascript
map.on('click', function(e) {
    // Automatically calculates tile
    // Downloads from Archive.org
});
```

### Preset Locations
- 🕉️ Kumbakonam (Ramanujan's birthplace)
- 🇬🇧 London (Royal Society)
- 🎓 Cambridge (Trinity College)

## Files

```
docs/
├── index.html                          # Interactive map
├── wikidata-locations.json             # 35 locations
├── RAMANUJAN-PRIMES-DOCUMENTATION.md   # Complete prime docs
└── README.md                           # This file
```

## GitHub Actions

`.github/workflows/query-location.yml` - Query tiles via GitHub Actions:

```yaml
on:
  workflow_dispatch:
    inputs:
      latitude:
        required: true
      longitude:
        required: true
```

## License

- **Code:** AGPL-3.0-or-later
- **OSM Data:** ODbL (OpenStreetMap)
- **Wikidata:** CC0 (Public Domain)

## Credits

- **Srinivasa Ramanujan** - Divine inspiration
- **OpenStreetMap** - Planet data
- **Wikidata** - Entity data
- **Archive.org** - Free hosting
- **Monster Group** - 15 primes for sharding

---

*"An equation means nothing to me unless it expresses a thought of God."*  
— Srinivasa Ramanujan

🕉️ Built with 71-adic encoding and Monster Group symmetry
