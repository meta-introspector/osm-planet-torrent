#!/usr/bin/env bash
# Build doors using ONLY small chunks from Archive.org torrents

set -e

ARCHIVE_BASE="https://archive.org/download"
OUTPUT_DIR="doors/door-70-subdoors"
mkdir -p "$OUTPUT_DIR"

# Function to fetch small metadata chunk (< 10KB)
fetch_metadata() {
    local dataset=$1
    local url="${ARCHIVE_BASE}/osm-planet-${dataset}-monster/${dataset}_files.xml"
    echo "Fetching metadata for $dataset..."
    curl -sL "$url" | head -100 > "/tmp/${dataset}_meta.xml" || echo "<!-- No metadata -->"
}

# Function to fetch tiny data sample (< 50KB)
fetch_sample() {
    local dataset=$1
    local file=$2
    local url="${ARCHIVE_BASE}/osm-planet-${dataset}-monster/${file}"
    echo "Fetching sample from $dataset/$file..."
    curl -sL "$url" --max-filesize 50000 2>/dev/null | head -1000 > "/tmp/${dataset}_sample.txt" || echo "# No sample"
}

# Build subdoor 5 - Archive.org Live Data
cat > "$OUTPUT_DIR/subdoor-5.html" << 'EOHTML'
<!DOCTYPE html>
<html>
<head>
    <title>Subdoor 5 - Archive.org Torrents</title>
    <meta charset="utf-8">
    <style>
        body { margin: 0; padding: 20px; font-family: monospace; background: #000; color: #0f0; }
        .container { max-width: 900px; margin: 0 auto; }
        h1 { color: #0ff; }
        .dataset { background: #111; padding: 15px; margin: 10px 0; border: 1px solid #0f0; }
        .dataset h3 { color: #ff0; margin: 0 0 10px; }
        .torrent-link { color: #0ff; text-decoration: none; }
        .torrent-link:hover { color: #fff; }
        .stats { color: #888; font-size: 12px; }
        .back { margin-top: 20px; }
        .back a { color: #0f0; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🌊 Subdoor 5 - Archive.org Torrents</h1>
        <p>22 datasets available via BitTorrent (selective download)</p>
        
        <div id="datasets"></div>
        
        <div class="back">
            <a href="index.html">← Back to Door 70 Subdoors</a>
        </div>
    </div>
    
    <script>
        const datasets = [
            {name: "pacific", desc: "Pacific Ocean nodes", shard: 0},
            {name: "himalayas", desc: "Himalayan peaks", shard: 5},
            {name: "amazon", desc: "Amazon rainforest", shard: 10},
            {name: "giza", desc: "Giza Pyramids (Cusp 17)", shard: 17},
            {name: "silicon", desc: "Silicon Valley (Cusp 23)", shard: 23},
            {name: "newyork", desc: "New York City", shard: 30},
            {name: "tokyo", desc: "Tokyo Metro", shard: 35},
            {name: "london", desc: "London", shard: 40},
            {name: "ramanujan", desc: "Ramanujan Temple (Cusp 59)", shard: 59},
            {name: "omega", desc: "Omega Point", shard: 70}
        ];
        
        const html = datasets.map(d => `
            <div class="dataset">
                <h3>📦 Shard ${d.shard}: ${d.name}</h3>
                <p>${d.desc}</p>
                <div class="stats">
                    <a class="torrent-link" href="https://archive.org/download/osm-planet-${d.name}-monster/${d.name}_archive.torrent">
                        ⬇️ Download Torrent
                    </a> | 
                    <a class="torrent-link" href="https://archive.org/details/osm-planet-${d.name}-monster">
                        📊 View on Archive.org
                    </a>
                </div>
            </div>
        `).join('');
        
        document.getElementById('datasets').innerHTML = html;
    </script>
</body>
</html>
EOHTML

echo "✅ Subdoor 5 created (Archive.org torrents)"

# Build subdoor 6 - Torrent Downloader
cat > "$OUTPUT_DIR/subdoor-6.html" << 'EOHTML'
<!DOCTYPE html>
<html>
<head>
    <title>Subdoor 6 - Selective Torrent Download</title>
    <meta charset="utf-8">
    <style>
        body { margin: 0; padding: 20px; font-family: monospace; background: #1a1a1a; color: #fff; }
        .container { max-width: 800px; margin: 0 auto; }
        h1 { color: #4CAF50; }
        .download-form { background: #2a2a2a; padding: 20px; border-radius: 8px; margin: 20px 0; }
        input, select { padding: 10px; margin: 5px; width: 200px; background: #333; color: #fff; border: 1px solid #555; }
        button { padding: 10px 20px; background: #4CAF50; color: white; border: none; cursor: pointer; margin: 5px; }
        button:hover { background: #45a049; }
        .output { background: #000; padding: 15px; margin: 10px 0; border: 1px solid #4CAF50; min-height: 100px; }
        .back a { color: #4CAF50; }
    </style>
</head>
<body>
    <div class="container">
        <h1>⚡ Subdoor 6 - Selective Download</h1>
        <p>Download only the chunks you need (< 50KB each)</p>
        
        <div class="download-form">
            <h3>Select Location:</h3>
            <select id="shard">
                <option value="17">Shard 17 - Giza Pyramids</option>
                <option value="23">Shard 23 - Silicon Valley</option>
                <option value="59">Shard 59 - Ramanujan Temple</option>
            </select>
            <br>
            <label>Latitude: <input type="number" id="lat" value="29.9792" step="0.0001"></label>
            <label>Longitude: <input type="number" id="lon" value="31.1342" step="0.0001"></label>
            <br>
            <button onclick="downloadChunk()">📥 Download Chunk</button>
        </div>
        
        <div class="output" id="output">Ready to download...</div>
        
        <div class="back">
            <a href="index.html">← Back to Door 70 Subdoors</a>
        </div>
    </div>
    
    <script>
        function downloadChunk() {
            const shard = document.getElementById('shard').value;
            const lat = document.getElementById('lat').value;
            const lon = document.getElementById('lon').value;
            
            const output = document.getElementById('output');
            output.innerHTML = `
                <strong>Downloading chunk...</strong><br>
                Shard: ${shard}<br>
                Location: [${lat}, ${lon}]<br>
                <br>
                <em>In production, this would:</em><br>
                1. Calculate tile: (${lat}, ${lon}) → tile_${Math.floor((parseFloat(lat) + 90) % 71)}_${Math.floor((parseFloat(lon) + 180) % 59)}<br>
                2. Fetch from Archive.org: ~4KB chunk<br>
                3. Parse GeoJSON nodes<br>
                4. Display on map<br>
                <br>
                <a href="https://archive.org/download/osm-planet-giza-monster/" style="color: #4CAF50;">
                    View full dataset on Archive.org →
                </a>
            `;
        }
    </script>
</body>
</html>
EOHTML

echo "✅ Subdoor 6 created (Selective downloader)"

echo ""
echo "✅ Build complete!"
echo "Created:"
echo "  - subdoor-5.html (Archive.org torrents)"
echo "  - subdoor-6.html (Selective downloader)"
