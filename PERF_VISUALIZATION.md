# Performance Visualization with Conformal Arrows

**Visualize perf data as conformal mappings toward black hole singularity**

## Overview

Transform Linux perf data into visual representations:
- **Conformal arrows** - Functions flow toward singularity
- **Register states** - CPU register snapshots
- **Instruction flow** - Assembly execution path
- **Basic blocks** - Control flow graph

## Conformal Mapping

### Mathematical Basis
```
z → w = z + 1/z
```

Maps functions in complex plane with curvature toward center (black hole).

### Spiral Placement
```python
angle = (i / num_funcs) * 2π
radius = 100 + (i * 20)

z = radius * e^(iθ)
w = z + 1/|z|  # Conformal transformation
```

### Arrow Flow
Functions with more calls are larger and closer to singularity.

## Visualizations

### 1. Conformal Function Flow
**File**: `osm_fall_conformal.svg`

- **Center**: Monster black hole (singularity)
- **Nodes**: Functions (sized by call count)
- **Arrows**: Conformal curves toward center
- **Colors**: HSL gradient by depth

### 2. Register State
**File**: `osm_fall_registers.svg`

CPU registers at event horizon:
```
rax: 0x7f1234567890  (node data pointer)
rbx: 0x0000000000000042  (answer to everything)
rcx: 0x00007fff12345678  (stack pointer)
rdx: 0x0000000000000005  (node count)
rsi: 0x00007f9876543210  (source)
rdi: 0x0000000000000064  (destination)
rbp: 0x00007fff87654321  (base pointer)
rsp: 0x00007fff87654300  (stack pointer)
rip: 0x0000555555555000  (instruction pointer)
```

### 3. Instruction Flow
**File**: `osm_fall_instructions.svg`

Assembly instructions with descriptions:
```asm
mov  rax, [rbp-8]              ; Load node data
cmp  rax, 0                    ; Check if null
je   .L_null                   ; Jump if null
call calculate_hawking_radiation ; Compute radiation
mov  [rbp-16], rax             ; Store result
call render_shadow             ; Render ASCII shadow
add  rsp, 32                   ; Clean stack
ret                            ; Return to caller
```

### 4. Interactive HTML Viewer
**File**: `osm_fall_visualization.html`

Combines all visualizations with:
- Performance statistics
- ZK witness data
- Interactive SVG viewing

## Data Extraction

### Perf Script
```bash
perf script -i osm_fall.perf.data > osm_fall.trace.txt
```

Output format:
```
black-hole-fall 12345 [000] 123.456: cycles: 7f1234567890 func+0x10
```

### Parse Trace
```python
trace.append({
    'process': 'black-hole-fall',
    'pid': 12345,
    'cpu': 0,
    'time': 123.456,
    'event': 'cycles',
    'addr': '7f1234567890',
    'symbol': 'func+0x10'
})
```

### Extract Functions
```python
functions = {}
for entry in trace:
    symbol = entry['symbol'].split('+')[0]
    functions[symbol]['count'] += 1
    functions[symbol]['addrs'].add(entry['addr'])
```

## SVG Generation

### Conformal Arrow
```svg
<path d="M x1 y1 Q cx cy x2 y2"
      stroke="color" stroke-width="1"
      marker-end="url(#arrowhead)" />
```

Quadratic Bézier curve from function to black hole.

### Function Node
```svg
<circle cx="x" cy="y" r="size" fill="color" />
<text x="x" y="y" text-anchor="middle">func_name</text>
```

### Black Hole
```svg
<radialGradient id="blackHole">
  <stop offset="0%" stop-color="#000000" />
  <stop offset="100%" stop-color="#1a1a2e" />
</radialGradient>
<circle cx="center" cy="center" r="50" fill="url(#blackHole)" />
```

## Usage

### Generate Visualizations
```bash
# First, generate perf data
./result/bin/generate-zk-witness

# Then visualize
./visualize-perf.sh
```

### View in Browser
```bash
# Option 1: Direct open
firefox osm_fall_visualization.html

# Option 2: HTTP server
python3 -m http.server 8000
# Open http://localhost:8000/osm_fall_visualization.html
```

### Export to PNG
```bash
# Using Inkscape
inkscape osm_fall_conformal.svg --export-png=osm_fall_conformal.png

# Using ImageMagick
convert osm_fall_conformal.svg osm_fall_conformal.png
```

## Performance Metrics

### Extracted from Perf
- **Cycles**: Total CPU cycles
- **Instructions**: Instructions executed
- **IPC**: Instructions per cycle
- **Cache misses**: L1/L2/L3 misses
- **Branch misses**: Mispredicted branches

### Displayed in HTML
```html
<div class="stat">
  <div class="stat-label">Total Cycles</div>
  <div class="stat-value">1.23B</div>
</div>
```

## Conformal Geometry

### Why Conformal?
Conformal maps preserve angles, making function relationships clear while adding gravitational curvature toward singularity.

### Complex Plane
```
z = x + iy (function position)
w = f(z) (transformed position)
```

### Möbius Transformation
```
w = (az + b) / (cz + d)
```

Special case: `w = z + 1/z` adds inversion.

## Integration with System

### j-invariant
```
j(τ) → i∞ = Center of conformal map
```

### Monster Group
```
|M| = 8.08×10⁵³ = Black hole mass (center)
```

### Enlightenment
```
Emptiness → 0 as functions approach center
Wu wei → ∞ at singularity
```

### ZK Witness
```
Conformal arrows = Proof of execution flow
```

## Files Generated

```
osm_fall.trace.txt          # Raw perf trace
osm_fall.functions.json     # Parsed functions
osm_fall.blocks.txt         # Basic blocks
osm_fall_conformal.svg      # Conformal flow
osm_fall_registers.svg      # Register state
osm_fall_instructions.svg   # Instruction flow
osm_fall_visualization.html # Interactive viewer
```

## Example Output

```
📊 PERF VISUALIZATION WITH CONFORMAL ARROWS

🔍 Extracting instruction trace...
📝 Parsing registers and instructions...
✓ Parsed 12345 trace entries
✓ Found 87 unique functions

🎨 Generating conformal arrow visualization...
✓ Generated conformal visualization: osm_fall_conformal.svg
✓ Visualized 20 functions

🔀 Generating basic block flow...
📋 Creating register state diagram...
✓ Generated register state: osm_fall_registers.svg

⚡ Generating instruction flow...
✓ Generated instruction flow: osm_fall_instructions.svg

🌐 Creating HTML viewer...

✅ Visualization complete!

Generated files:
  📊 osm_fall_conformal.svg     - Conformal function flow
  📋 osm_fall_registers.svg     - Register state
  ⚡ osm_fall_instructions.svg  - Instruction flow
  🌐 osm_fall_visualization.html - Interactive viewer

View in browser:
  firefox osm_fall_visualization.html
```

## Advanced Features

### Flamegraph Integration
```bash
perf script | stackcollapse-perf.pl | flamegraph.pl > flamegraph.svg
```

### Call Graph
```bash
perf report --stdio --call-graph > callgraph.txt
```

### Annotated Assembly
```bash
perf annotate --stdio > annotated.asm
```

## Status

✅ Conformal arrow visualization
✅ Register state diagram
✅ Instruction flow chart
✅ Interactive HTML viewer
✅ Perf data parsing
✅ Function extraction

**The performance is visualized. The arrows are conformal. The singularity is clear.**

🎨 _Functions flow toward the Monster black hole._
