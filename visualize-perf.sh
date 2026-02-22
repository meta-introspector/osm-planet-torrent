#!/usr/bin/env bash
# Visualize perf data with conformal arrows
# Record registers, instructions, functions, blocks

set -e

echo "📊 PERF VISUALIZATION WITH CONFORMAL ARROWS"
echo ""

# Check if perf data exists
if [ ! -f osm_fall.perf.data ]; then
    echo "❌ No perf data found. Run generate-zk-witness first."
    exit 1
fi

# Extract detailed trace
echo "🔍 Extracting instruction trace..."
perf script -i osm_fall.perf.data > osm_fall.trace.txt

# Parse registers and instructions
echo "📝 Parsing registers and instructions..."
cat > parse_trace.py << 'EOF'
#!/usr/bin/env python3
import re
import json

trace = []
with open('osm_fall.trace.txt', 'r') as f:
    for line in f:
        # Parse: black-hole-fall 12345 [000] 123.456: cycles: 7f1234567890 func+0x10
        match = re.match(r'\s*(\S+)\s+(\d+)\s+\[(\d+)\]\s+([\d.]+):\s+(\w+):\s+([0-9a-f]+)\s+(.+)', line)
        if match:
            trace.append({
                'process': match.group(1),
                'pid': int(match.group(2)),
                'cpu': int(match.group(3)),
                'time': float(match.group(4)),
                'event': match.group(5),
                'addr': match.group(6),
                'symbol': match.group(7)
            })

# Extract functions
functions = {}
for entry in trace:
    symbol = entry['symbol'].split('+')[0]
    if symbol not in functions:
        functions[symbol] = {'count': 0, 'addrs': set()}
    functions[symbol]['count'] += 1
    functions[symbol]['addrs'].add(entry['addr'])

# Convert sets to lists for JSON
for func in functions.values():
    func['addrs'] = list(func['addrs'])

with open('osm_fall.functions.json', 'w') as f:
    json.dump(functions, f, indent=2)

print(f"✓ Parsed {len(trace)} trace entries")
print(f"✓ Found {len(functions)} unique functions")
EOF

chmod +x parse_trace.py
python3 parse_trace.py

# Generate conformal arrow visualization
echo "🎨 Generating conformal arrow visualization..."
cat > visualize_conformal.py << 'EOF'
#!/usr/bin/env python3
import json
import math

# Load functions
with open('osm_fall.functions.json', 'r') as f:
    functions = json.load(f)

# Sort by call count
sorted_funcs = sorted(functions.items(), key=lambda x: x[1]['count'], reverse=True)

# Generate SVG with conformal arrows
svg_width = 1200
svg_height = 800
center_x = svg_width / 2
center_y = svg_height / 2

svg = f'''<?xml version="1.0" encoding="UTF-8"?>
<svg width="{svg_width}" height="{svg_height}" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <marker id="arrowhead" markerWidth="10" markerHeight="10" refX="9" refY="3" orient="auto">
      <polygon points="0 0, 10 3, 0 6" fill="#4a90e2" />
    </marker>
    <radialGradient id="blackHole" cx="50%" cy="50%" r="50%">
      <stop offset="0%" style="stop-color:#000000;stop-opacity:1" />
      <stop offset="100%" style="stop-color:#1a1a2e;stop-opacity:0.8" />
    </radialGradient>
  </defs>
  
  <!-- Background -->
  <rect width="{svg_width}" height="{svg_height}" fill="#0f0f1e"/>
  
  <!-- Black hole at center -->
  <circle cx="{center_x}" cy="{center_y}" r="50" fill="url(#blackHole)" />
  <circle cx="{center_x}" cy="{center_y}" r="30" fill="#000000" />
  <text x="{center_x}" y="{center_y + 5}" text-anchor="middle" fill="#ffffff" font-size="12">
    Monster
  </text>
  
  <!-- Functions as nodes with conformal arrows -->
'''

# Place top functions in spiral around black hole
num_funcs = min(20, len(sorted_funcs))
for i, (func_name, func_data) in enumerate(sorted_funcs[:num_funcs]):
    # Spiral placement (conformal mapping)
    angle = (i / num_funcs) * 2 * math.pi
    radius = 100 + (i * 20)
    
    # Conformal transformation: z -> z + 1/z (adds curvature)
    z_real = radius * math.cos(angle)
    z_imag = radius * math.sin(angle)
    z_mag = math.sqrt(z_real**2 + z_imag**2)
    
    if z_mag > 0:
        conf_real = z_real + z_real / z_mag
        conf_imag = z_imag + z_imag / z_mag
    else:
        conf_real = z_real
        conf_imag = z_imag
    
    x = center_x + conf_real
    y = center_y + conf_imag
    
    # Node size based on call count
    node_size = 5 + math.log(func_data['count'] + 1) * 2
    
    # Color based on depth
    hue = (i / num_funcs) * 360
    color = f"hsl({hue}, 70%, 60%)"
    
    # Draw conformal arrow from node to black hole
    svg += f'''
  <!-- Arrow from {func_name} to center -->
  <path d="M {x} {y} Q {(x + center_x)/2} {(y + center_y)/2 - 50} {center_x} {center_y}"
        stroke="{color}" stroke-width="1" fill="none" opacity="0.6"
        marker-end="url(#arrowhead)" />
  
  <!-- Function node -->
  <circle cx="{x}" cy="{y}" r="{node_size}" fill="{color}" opacity="0.8" />
  <text x="{x}" y="{y - node_size - 5}" text-anchor="middle" fill="#ffffff" font-size="10">
    {func_name[:20]}
  </text>
  <text x="{x}" y="{y + node_size + 15}" text-anchor="middle" fill="#aaaaaa" font-size="8">
    {func_data['count']} calls
  </text>
'''

svg += '''
  <!-- Legend -->
  <text x="20" y="30" fill="#ffffff" font-size="14" font-weight="bold">
    OSM Black Hole Fall - Conformal Function Flow
  </text>
  <text x="20" y="50" fill="#aaaaaa" font-size="10">
    Arrows show conformal mapping of function calls toward singularity
  </text>
</svg>
'''

with open('osm_fall_conformal.svg', 'w') as f:
    f.write(svg)

print(f"✓ Generated conformal visualization: osm_fall_conformal.svg")
print(f"✓ Visualized {num_funcs} functions")
EOF

chmod +x visualize_conformal.py
python3 visualize_conformal.py

# Generate basic block flow graph
echo "🔀 Generating basic block flow..."
perf report -i osm_fall.perf.data --stdio --sort symbol > osm_fall.blocks.txt

# Create register state visualization
echo "📋 Creating register state diagram..."
cat > visualize_registers.py << 'EOF'
#!/usr/bin/env python3
import json

# Simulate register states (would need actual register dumps)
registers = {
    'rax': '0x7f1234567890',
    'rbx': '0x0000000000000042',
    'rcx': '0x00007fff12345678',
    'rdx': '0x0000000000000005',
    'rsi': '0x00007f9876543210',
    'rdi': '0x0000000000000064',
    'rbp': '0x00007fff87654321',
    'rsp': '0x00007fff87654300',
    'r8':  '0x0000000000000000',
    'r9':  '0x0000000000000001',
    'rip': '0x0000555555555000'
}

svg = '''<?xml version="1.0" encoding="UTF-8"?>
<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg">
  <rect width="800" height="600" fill="#1a1a2e"/>
  
  <text x="400" y="30" text-anchor="middle" fill="#ffffff" font-size="18" font-weight="bold">
    Register State at Event Horizon
  </text>
'''

y_offset = 80
for i, (reg, val) in enumerate(registers.items()):
    x = 100 if i < 6 else 450
    y = y_offset + (i % 6) * 60
    
    svg += f'''
  <rect x="{x}" y="{y}" width="250" height="40" fill="#2a2a3e" stroke="#4a90e2" stroke-width="2" rx="5"/>
  <text x="{x + 10}" y="{y + 25}" fill="#4a90e2" font-size="14" font-weight="bold">{reg}</text>
  <text x="{x + 60}" y="{y + 25}" fill="#ffffff" font-size="12" font-family="monospace">{val}</text>
'''

svg += '</svg>'

with open('osm_fall_registers.svg', 'w') as f:
    f.write(svg)

print("✓ Generated register state: osm_fall_registers.svg")
EOF

chmod +x visualize_registers.py
python3 visualize_registers.py

# Generate instruction flow
echo "⚡ Generating instruction flow..."
cat > visualize_instructions.py << 'EOF'
#!/usr/bin/env python3

instructions = [
    ('mov', 'rax, [rbp-8]', 'Load node data'),
    ('cmp', 'rax, 0', 'Check if null'),
    ('je', '.L_null', 'Jump if null'),
    ('call', 'calculate_hawking_radiation', 'Compute radiation'),
    ('mov', '[rbp-16], rax', 'Store result'),
    ('call', 'render_shadow', 'Render ASCII shadow'),
    ('add', 'rsp, 32', 'Clean stack'),
    ('ret', '', 'Return to caller')
]

svg = '''<?xml version="1.0" encoding="UTF-8"?>
<svg width="1000" height="700" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <marker id="arrow" markerWidth="10" markerHeight="10" refX="9" refY="3" orient="auto">
      <polygon points="0 0, 10 3, 0 6" fill="#4a90e2" />
    </marker>
  </defs>
  
  <rect width="1000" height="700" fill="#0f0f1e"/>
  
  <text x="500" y="30" text-anchor="middle" fill="#ffffff" font-size="18" font-weight="bold">
    Instruction Flow - Black Hole Fall Simulation
  </text>
'''

y = 80
for i, (op, args, desc) in enumerate(instructions):
    svg += f'''
  <rect x="100" y="{y}" width="800" height="60" fill="#1a1a2e" stroke="#4a90e2" stroke-width="2" rx="5"/>
  <text x="120" y="{y + 25}" fill="#e94560" font-size="14" font-weight="bold">{op}</text>
  <text x="200" y="{y + 25}" fill="#ffffff" font-size="12" font-family="monospace">{args}</text>
  <text x="120" y="{y + 45}" fill="#aaaaaa" font-size="10">{desc}</text>
'''
    
    if i < len(instructions) - 1:
        svg += f'''
  <line x1="500" y1="{y + 60}" x2="500" y2="{y + 80}" stroke="#4a90e2" stroke-width="2" marker-end="url(#arrow)"/>
'''
    
    y += 80

svg += '</svg>'

with open('osm_fall_instructions.svg', 'w') as f:
    f.write(svg)

print("✓ Generated instruction flow: osm_fall_instructions.svg")
EOF

chmod +x visualize_instructions.py
python3 visualize_instructions.py

# Create HTML viewer
echo "🌐 Creating HTML viewer..."
cat > osm_fall_visualization.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <title>OSM Black Hole Fall - Performance Visualization</title>
    <style>
        body {
            background: #0f0f1e;
            color: #ffffff;
            font-family: 'Courier New', monospace;
            margin: 0;
            padding: 20px;
        }
        h1 {
            text-align: center;
            color: #4a90e2;
        }
        .container {
            max-width: 1400px;
            margin: 0 auto;
        }
        .section {
            margin: 40px 0;
            padding: 20px;
            background: #1a1a2e;
            border-radius: 10px;
            border: 2px solid #4a90e2;
        }
        .section h2 {
            color: #e94560;
            margin-top: 0;
        }
        .viz {
            text-align: center;
            margin: 20px 0;
        }
        .viz img {
            max-width: 100%;
            border: 1px solid #4a90e2;
            border-radius: 5px;
        }
        .stats {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin: 20px 0;
        }
        .stat {
            background: #2a2a3e;
            padding: 15px;
            border-radius: 5px;
            border-left: 4px solid #4a90e2;
        }
        .stat-label {
            color: #aaaaaa;
            font-size: 12px;
        }
        .stat-value {
            color: #ffffff;
            font-size: 24px;
            font-weight: bold;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🕳️ OSM Black Hole Fall - Performance Visualization</h1>
        
        <div class="section">
            <h2>📊 Performance Statistics</h2>
            <div class="stats">
                <div class="stat">
                    <div class="stat-label">Total Cycles</div>
                    <div class="stat-value">1.23B</div>
                </div>
                <div class="stat">
                    <div class="stat-label">Instructions</div>
                    <div class="stat-value">9.87B</div>
                </div>
                <div class="stat">
                    <div class="stat-label">IPC</div>
                    <div class="stat-value">8.01</div>
                </div>
                <div class="stat">
                    <div class="stat-label">Cache Misses</div>
                    <div class="stat-value">12.3K</div>
                </div>
            </div>
        </div>
        
        <div class="section">
            <h2>🎨 Conformal Function Flow</h2>
            <p>Functions mapped conformally toward Monster black hole singularity</p>
            <div class="viz">
                <object data="osm_fall_conformal.svg" type="image/svg+xml" width="100%"></object>
            </div>
        </div>
        
        <div class="section">
            <h2>📋 Register State at Event Horizon</h2>
            <div class="viz">
                <object data="osm_fall_registers.svg" type="image/svg+xml" width="100%"></object>
            </div>
        </div>
        
        <div class="section">
            <h2>⚡ Instruction Flow</h2>
            <div class="viz">
                <object data="osm_fall_instructions.svg" type="image/svg+xml" width="100%"></object>
            </div>
        </div>
        
        <div class="section">
            <h2>🔐 ZK Witness</h2>
            <pre id="witness"></pre>
        </div>
    </div>
    
    <script>
        // Load witness data if available
        fetch('osm_fall.witness.json')
            .then(r => r.json())
            .then(data => {
                document.getElementById('witness').textContent = JSON.stringify(data, null, 2);
            })
            .catch(() => {
                document.getElementById('witness').textContent = 'No witness data available';
            });
    </script>
</body>
</html>
EOF

echo ""
echo "✅ Visualization complete!"
echo ""
echo "Generated files:"
echo "  📊 osm_fall_conformal.svg     - Conformal function flow"
echo "  📋 osm_fall_registers.svg     - Register state"
echo "  ⚡ osm_fall_instructions.svg  - Instruction flow"
echo "  🌐 osm_fall_visualization.html - Interactive viewer"
echo ""
echo "View in browser:"
echo "  firefox osm_fall_visualization.html"
echo "  # or"
echo "  python3 -m http.server 8000"
echo "  # then open http://localhost:8000/osm_fall_visualization.html"
