# Solana Tile Service Update Plan

## Current Status
- **URL**: https://solana.solfunmeme.com/tiles/
- **Backend**: Needs update to serve Monster-compressed tiles

## New Architecture

### Tile Formats

**1. Standard Tiles** (backward compatible)
```
GET /tiles/{z}/{x}/{y}.png
GET /tiles/{z}/{x}/{y}.json
```

**2. Monster Emoji Tiles** (NEW)
```
GET /tiles/{z}/{x}/{y}.emoji
Response: 24×24 emoji grid (150 bytes compressed)
```

**3. Monster Metadata** (NEW)
```
GET /tiles/{z}/{x}/{y}.monster
Response: {
  "shard_id": 17,
  "monster_shadow": 16294812390,
  "fitness_score": 23.51,
  "conjugacy_class": "17A",
  "gielis_angle": 1.5044,
  "hecke_states": [...]
}
```

**4. Gielis Visualization** (NEW)
```
GET /tiles/gielis/{shard_id}
Response: SVG of 71-fold pattern for shard
```

**5. The Cusp** (NEW, Premium)
```
GET /tiles/cusp
Response: Shard 17 with 2832× resonance visualization
```

### Implementation

**Backend Service** (Rust + Axum):
```rust
// tile-service/src/main.rs
use axum::{Router, routing::get, Json};
use monster_osm::*;

#[tokio::main]
async fn main() {
    let app = Router::new()
        // Legacy endpoints
        .route("/tiles/:z/:x/:y.png", get(serve_png))
        .route("/tiles/:z/:x/:y.json", get(serve_json))
        
        // Monster endpoints
        .route("/tiles/:z/:x/:y.emoji", get(serve_emoji))
        .route("/tiles/:z/:x/:y.monster", get(serve_monster))
        .route("/tiles/gielis/:shard", get(serve_gielis))
        .route("/tiles/cusp", get(serve_cusp))
        
        // Health check
        .route("/health", get(|| async { "OK" }));
    
    let addr = "0.0.0.0:3000".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn serve_emoji(
    Path((z, x, y)): Path<(u8, u32, u32)>,
) -> Result<Vec<u8>, StatusCode> {
    let cube = EmojiCube::load(z, x, y)?;
    Ok(cube.compress())
}

async fn serve_monster(
    Path((z, x, y)): Path<(u8, u32, u32)>,
) -> Result<Json<MonsterMetadata>, StatusCode> {
    let meta = MonsterMetadata::load(z, x, y)?;
    Ok(Json(meta))
}
```

### Deployment

**1. Build Service**:
```bash
cd /home/mdupont/projects/osm-planet-torrent
mkdir -p tile-service/src
cd tile-service

# Create Cargo.toml
cat > Cargo.toml <<EOF
[package]
name = "tile-service"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
monster-osm = { path = "../monster-osm" }
EOF

# Build
cargo build --release
```

**2. Deploy to Server**:
```bash
# Copy binary
scp target/release/tile-service solana.solfunmeme.com:/opt/tiles/

# Create systemd service
ssh solana.solfunmeme.com
sudo tee /etc/systemd/system/tile-service.service <<EOF
[Unit]
Description=Monster OSM Tile Service
After=network.target

[Service]
Type=simple
User=tiles
WorkingDirectory=/opt/tiles
ExecStart=/opt/tiles/tile-service
Restart=always

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl enable tile-service
sudo systemctl start tile-service
```

**3. Update Nginx**:
```nginx
# /etc/nginx/sites-available/solana.solfunmeme.com
server {
    listen 443 ssl;
    server_name solana.solfunmeme.com;
    
    location /tiles/ {
        proxy_pass http://localhost:3000/tiles/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        
        # Cache emoji tiles (they're immutable)
        location ~ \.emoji$ {
            proxy_cache tiles_cache;
            proxy_cache_valid 200 30d;
            add_header X-Cache-Status $upstream_cache_status;
        }
    }
}
```

### Frontend Updates

**JavaScript Client**:
```javascript
// Load emoji tile
async function loadEmojiTile(z, x, y) {
    const response = await fetch(
        `https://solana.solfunmeme.com/tiles/${z}/${x}/${y}.emoji`
    );
    const data = await response.arrayBuffer();
    return decodeEmojiCube(data);
}

// Load Monster metadata
async function loadMonsterMetadata(z, x, y) {
    const response = await fetch(
        `https://solana.solfunmeme.com/tiles/${z}/${x}/${y}.monster`
    );
    return await response.json();
}

// Render emoji tile
function renderEmojiTile(canvas, emojiCube) {
    const ctx = canvas.getContext('2d');
    const size = 24;
    const cellSize = canvas.width / size;
    
    for (let y = 0; y < size; y++) {
        for (let x = 0; x < size; x++) {
            const emoji = emojiCube[y][x];
            ctx.font = `${cellSize}px sans-serif`;
            ctx.fillText(emoji, x * cellSize, (y + 1) * cellSize);
        }
    }
}

// Visualize Gielis pattern
async function showGielisPattern(shardId) {
    const response = await fetch(
        `https://solana.solfunmeme.com/tiles/gielis/${shardId}`
    );
    const svg = await response.text();
    document.getElementById('gielis-viz').innerHTML = svg;
}
```

### Solana Integration (Phase 2)

**Access Control Program**:
```rust
// Anchor program for tile access
#[program]
pub mod tile_access {
    use super::*;
    
    pub fn purchase_access(
        ctx: Context<PurchaseAccess>,
        shard_id: u8,
    ) -> Result<()> {
        require!(shard_id < 71, ErrorCode::InvalidShard);
        
        let cost = match shard_id {
            0 => 0,              // Free (identity)
            17 => 10_000_000,    // 0.01 SOL (The Cusp)
            _ => 1_000_000,      // 0.001 SOL (regular)
        };
        
        // Transfer SOL
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.payer.key(),
            &ctx.accounts.treasury.key(),
            cost,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.treasury.to_account_info(),
            ],
        )?;
        
        // Grant access
        ctx.accounts.access_token.shard_id = shard_id;
        ctx.accounts.access_token.expires_at = Clock::get()?.unix_timestamp + 86400;
        
        Ok(())
    }
}
```

## Testing

```bash
# 1. Start local service
cd /home/mdupont/projects/osm-planet-torrent/tile-service
cargo run

# 2. Test endpoints
curl http://localhost:3000/tiles/0/0/0.emoji
curl http://localhost:3000/tiles/0/0/0.monster
curl http://localhost:3000/tiles/gielis/17
curl http://localhost:3000/tiles/cusp

# 3. Load test
ab -n 1000 -c 10 http://localhost:3000/tiles/0/0/0.emoji
```

## Monitoring

**Metrics to Track**:
- Requests per second
- Cache hit rate
- Average response time
- Emoji tile size distribution
- Most accessed shards
- Cusp (Shard 17) access frequency

**Grafana Dashboard**:
- Tile request heatmap
- 71-shard distribution
- Monster shadow histogram
- Gielis pattern popularity

## Timeline

**Week 1**: Build tile-service, test locally  
**Week 2**: Deploy to staging, integrate frontend  
**Week 3**: Production deployment  
**Week 4**: Solana program integration  

## Success Metrics

- [ ] 1000+ emoji tile requests/day
- [ ] <100ms average response time
- [ ] 90%+ cache hit rate
- [ ] 10+ Cusp (Shard 17) premium accesses
- [ ] 100+ unique Gielis visualizations viewed

---

**Status**: Ready to implement  
**Next**: Create tile-service skeleton  
**The tiles are alive on Solana.** 🌀💎
