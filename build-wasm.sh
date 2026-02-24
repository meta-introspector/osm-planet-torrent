#!/usr/bin/env bash
set -e

echo "🦀 Building Rust to WASM..."

# Install wasm-pack if needed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build WASM
wasm-pack build --target web --out-dir wasm-pkg

# Create minimal HTML with embedded WASM
cat > osm-torrent-wasm.html << 'HTML'
<!DOCTYPE html>
<html>
<head>
  <title>OSM Torrent Query - WASM</title>
  <meta charset="utf-8">
  <style>
    body { margin: 0; padding: 20px; font-family: monospace; background: #1a1a1a; color: #0f0; }
    .container { max-width: 800px; margin: 0 auto; }
    h1 { color: #0ff; }
    input, button { padding: 10px; margin: 5px; background: #000; color: #0f0; border: 1px solid #0f0; }
    button:hover { background: #0f0; color: #000; cursor: pointer; }
    .result { background: #000; padding: 15px; margin: 10px 0; border: 1px solid #0f0; }
    pre { color: #ff0; }
  </style>
</head>
<body>
  <div class="container">
    <h1>🦀 OSM Torrent Query (Rust WASM)</h1>
    
    <h3>Query Location</h3>
    <input type="number" id="lat" placeholder="Latitude" value="10.9617" step="0.0001">
    <input type="number" id="lon" placeholder="Longitude" value="79.3881" step="0.0001">
    <input type="text" id="name" placeholder="Name" value="Kumbakonam">
    <button onclick="queryLocation()">Query</button>
    
    <h3>Calculate Reduction</h3>
    <input type="number" id="fetched" placeholder="Fetched (MB)" value="4">
    <input type="number" id="total" placeholder="Total (GB)" value="86">
    <button onclick="calcReduction()">Calculate</button>
    
    <div class="result" id="result">
      <strong>Results will appear here...</strong>
    </div>
    
    <h3>Preloaded Locations</h3>
    <button onclick="loadKumbakonam()">Kumbakonam</button>
    <button onclick="loadChennai()">Chennai</button>
    <button onclick="loadCambridge()">Cambridge</button>
  </div>

  <script type="module">
    import init, { 
      query_location, 
      reduction_percent, 
      calculate_tile,
      calculate_shard,
      init as wasmInit
    } from './wasm-pkg/osm_planet_torrent.js';

    async function run() {
      await init();
      wasmInit();
      
      window.queryLocation = () => {
        const lat = parseFloat(document.getElementById('lat').value);
        const lon = parseFloat(document.getElementById('lon').value);
        const name = document.getElementById('name').value;
        
        const result = query_location(lat, lon, name);
        const data = JSON.parse(result);
        
        document.getElementById('result').innerHTML = `
          <strong>Query Result:</strong>
          <pre>${JSON.stringify(data, null, 2)}</pre>
          <p>Tile: ${data.tile}</p>
          <p>Piece: ${data.piece}</p>
          <p>Shard: ${data.shard} (mod 196,883)</p>
        `;
      };
      
      window.calcReduction = () => {
        const fetched = parseFloat(document.getElementById('fetched').value);
        const total = parseFloat(document.getElementById('total').value);
        
        const percent = reduction_percent(fetched, total);
        
        document.getElementById('result').innerHTML = `
          <strong>Reduction:</strong>
          <pre>Fetched: ${fetched}MB
Total: ${total}GB (${total * 1024}MB)
Reduction: ${percent.toFixed(3)}%</pre>
        `;
      };
      
      window.loadKumbakonam = () => {
        document.getElementById('lat').value = 10.9617;
        document.getElementById('lon').value = 79.3881;
        document.getElementById('name').value = 'Kumbakonam';
        queryLocation();
      };
      
      window.loadChennai = () => {
        document.getElementById('lat').value = 13.0827;
        document.getElementById('lon').value = 80.2707;
        document.getElementById('name').value = 'Chennai';
        queryLocation();
      };
      
      window.loadCambridge = () => {
        document.getElementById('lat').value = 52.2053;
        document.getElementById('lon').value = 0.1218;
        document.getElementById('name').value = 'Cambridge';
        queryLocation();
      };
      
      console.log('✅ WASM loaded');
    }

    run();
  </script>
</body>
</html>
HTML

echo "✅ Built WASM and created query interface"
ls -lh wasm-pkg/
