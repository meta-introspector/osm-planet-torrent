#!/usr/bin/env python3
# Query tiles for a location
import sys, json, glob

if len(sys.argv) != 3:
    print("Usage: query_tiles.py <lat> <lon>")
    sys.exit(1)

lat = float(sys.argv[1])
lon = float(sys.argv[2])

# Calculate tile (Z=8)
tile_x = int((lon + 180) / 360 * 256)
tile_y = int((90 - lat) / 180 * 256)

print(f"📍 Location: {lat}°N, {lon}°E")
print(f"🗺️  Tile: ({tile_x}, {tile_y})")
print()

# Check admin boundaries
print("🏛️ Admin boundaries:")
with open('admin/boundaries.jsonl') as f:
    for line in f:
        a = json.loads(line)
        # Simple proximity check (±0.5 degrees)
        if abs(a['lat'] - lat) < 0.5 and abs(a['lon'] - lon) < 0.5:
            print(f"  Level {a['level']}: {a['name']} (node {a['id']})")

print()

# List tile files
tile_dir = f"tiles/tile_{tile_x}_{tile_y}"
files = glob.glob(f"{tile_dir}/nodes_*.csv")

if files:
    print(f"📦 Tile contains {len(files)} node buckets:")
    total = 0
    for f in sorted(files):
        with open(f) as fp:
            count = sum(1 for _ in fp)
            total += count
            bucket = f.split('_')[-1].replace('.csv', '')
            print(f"  Bucket {bucket}: {count:,} nodes")
    print(f"\n✅ Total: {total:,} nodes in tile")
else:
    print(f"❌ Tile not found: {tile_dir}")
