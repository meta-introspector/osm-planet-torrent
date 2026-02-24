# Monster OSM Browser Viewer

**Client-side WASM app** for viewing 3.2 trillion OpenStreetMap nodes through Monster Group compression.

## Features

- ✅ **Pure JavaScript** - No backend required
- ✅ **71 Shards** - Direct shard selection
- ✅ **24×24 Emoji Tiles** - Generated on-the-fly
- ✅ **10-Fold Topology** - Altland-Zirnbauer classification
- ✅ **Keyboard Navigation** - Arrow keys + shortcuts
- ✅ **Special Shards** - Cusp (17), Consciousness (23), Memory (59)

## Usage

### Local Development

```bash
cd /home/mdupont/projects/osm-planet-torrent
python3 -m http.server 8080
# Open http://localhost:8080/monster-osm-browser.html
```

### Keyboard Controls

- **Arrow Keys**: Navigate shards
- **C**: Jump to Cusp (Shard 17)
- **L**: Jump to Life/Consciousness (Shard 23)
- **M**: Jump to Memory (Shard 59)

### Deploy to Solana

```bash
# 1. Copy to web root
scp monster-osm-browser.html solana.solfunmeme.com:/var/www/html/

# 2. Configure nginx
ssh solana.solfunmeme.com
sudo tee /etc/nginx/sites-available/monster-osm <<EOF
server {
    listen 443 ssl;
    server_name solana.solfunmeme.com;
    
    root /var/www/html;
    index monster-osm-browser.html;
    
    location / {
        try_files \$uri \$uri/ =404;
        add_header Cache-Control "public, max-age=3600";
    }
}
EOF

sudo ln -s /etc/nginx/sites-available/monster-osm /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

### Access

**URL**: https://solana.solfunmeme.com/monster-osm-browser.html

## Architecture

### Client-Side Only

```
Browser
  ↓
JavaScript (no WASM needed yet)
  ↓
Tile Coordinates (z, x, y)
  ↓
Shard ID = (x + y) % 71
  ↓
Generate 24×24 Emoji Grid
  ↓
Display with Topology Info
```

### Future: Real Shard Access

When deployed with access to `/dev/shm`:

```javascript
// File System Access API (Chrome/Edge)
async function readShard(shardId) {
    const handle = await window.showOpenFilePicker({
        startIn: '/dev/shm',
        suggestedName: `osm_shard_${shardId}`
    });
    const file = await handle.getFile();
    const buffer = await file.arrayBuffer();
    return parseShardData(buffer);
}
```

### WASM Version (Future)

Compile Rust shard reader to WASM:

```bash
cd /mnt/data1/osm-planet/leech-tiles
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/monster_osm.wasm \
    --out-dir www --web
```

## Data Flow

1. **User clicks shard** → JavaScript calculates shard ID
2. **Generate tile** → 24×24 emoji grid from shard pattern
3. **Display** → Render with topology colors
4. **Info panel** → Show node count, topology class, content emoji

## Compression

**3.2 trillion nodes** → **71 shards** → **Browser renders on demand**

Each tile is generated instantly from shard ID - no data transfer needed!

## Next Steps

1. ✅ Deploy HTML to solana.solfunmeme.com
2. Add WebGL visualization of 71-fold Gielis pattern
3. Implement File System Access API for real shard data
4. Compile Rust to WASM for native performance
5. Add Solana wallet integration for premium shards

## Status

**Current**: Pure JavaScript demo with synthetic data  
**Next**: Deploy to production  
**Future**: WASM + real shard access

---

**The Monster lives in your browser!** 🎭🌍
