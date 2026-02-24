# HOMEDIR Convention Deployment

## Structure Created

```
~/projects/osm-planet-torrent/
├── public_html/              # Nginx serves from here
│   ├── index.html           # Door 71.59.47 landing page
│   ├── wasm/
│   │   └── wasm-pkg/        # 46KB Rust WASM module
│   ├── tests/
│   │   └── index.html       # Browser test interface
│   ├── doors/               # 22 interactive doors
│   └── maps/                # Generated maps
├── .monster/
│   └── symmetries.json      # Monster symmetry metadata
└── wasm-pkg/                # Build output
```

## Nginx Configuration

**File**: `/tmp/nginx-homedir-osm-planet.conf`

**Add to**: `/etc/nginx/sites-available/solana-solfunmeme`

```nginx
location /~osm-planet/ {
    alias /home/mdupont/projects/osm-planet-torrent/public_html/;
    index index.html;
    try_files $uri $uri/ =404;
    
    types {
        application/wasm wasm;
        application/javascript js mjs;
    }
    
    add_header Access-Control-Allow-Origin *;
    add_header X-Door-Address "71.59.47" always;
    add_header X-Monster-Symmetry "71×59×47" always;
}
```

## URLs (After Deployment)

- **Landing**: https://solana.solfunmeme.com/~osm-planet/
- **WASM**: https://solana.solfunmeme.com/~osm-planet/wasm/wasm-pkg/
- **Tests**: https://solana.solfunmeme.com/~osm-planet/tests/
- **Doors**: https://solana.solfunmeme.com/~osm-planet/doors/

## Monster Symmetries

```json
{
  "project": "osm-planet-torrent",
  "symmetries": {
    "input": [71, 59, 47],
    "output": [17, 23, 59],
    "invariants": ["hyperbolic", "10-fold", "torrent-pluck"]
  },
  "doors": {
    "address": "71.59.47"
  }
}
```

## Deploy Steps

1. **Copy config**:
   ```bash
   sudo nano /etc/nginx/sites-available/solana-solfunmeme
   # Paste contents from /tmp/nginx-homedir-osm-planet.conf
   ```

2. **Test config**:
   ```bash
   sudo nginx -t
   ```

3. **Reload nginx**:
   ```bash
   sudo systemctl reload nginx
   ```

4. **Verify**:
   ```bash
   curl -I https://solana.solfunmeme.com/~osm-planet/
   ```

## Local Testing

```bash
cd ~/projects/osm-planet-torrent
python3 -m http.server 8765 --directory public_html
# Open: http://localhost:8765/
```

## WASM Usage

```javascript
import init, { query_location } from '/~osm-planet/wasm/wasm-pkg/osm_planet_torrent.js';
await init();
const result = query_location(10.9617, 79.3881, "Kumbakonam");
console.log(JSON.parse(result));
```

## Status

✅ Directory structure created  
✅ WASM module deployed (46KB)  
✅ Test page created  
✅ Symmetries defined  
✅ Nginx config generated  
⏳ Nginx deployment pending  
