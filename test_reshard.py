import glob, os

print("🏛️ Re-sharding Kumbakonam tile → Ramanujan (71×59×47)")

os.makedirs("ramanujan_tiles", exist_ok=True)

ram_tiles = {}
total = 0

for csv in glob.glob("test_tiles/tile_184_113/*.csv"):
    with open(csv) as f:
        for line in f:
            parts = line.strip().split(',')
            if len(parts) != 3: continue
            
            node_id, lat, lon = int(parts[0]), float(parts[1]), float(parts[2])
            
            tile_lat = int(((lat + 90) * 100) % 71)
            tile_lon = int(((lon + 180) * 100) % 59)
            tile_level = 0  # No admin_level in CSV
            
            k = (tile_lat, tile_lon, tile_level)
            
            if k not in ram_tiles:
                path = f"ramanujan_tiles/tile_{tile_lat:02d}_{tile_lon:02d}_{tile_level:02d}.csv"
                ram_tiles[k] = open(path, 'a')
            
            ram_tiles[k].write(f"{node_id},{lat:.7f},{lon:.7f}\n")
            total += 1

for f in ram_tiles.values():
    f.close()

print(f"✅ {total} nodes → {len(ram_tiles)} Ramanujan tiles")
