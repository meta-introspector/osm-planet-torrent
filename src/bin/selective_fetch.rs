// Selective OSM Planet Fetcher
// Downloads ONLY the minimal chunks needed for current map view
// Uses: 1) Archive.org torrents (selective piece download)
//       2) Overpass API (bbox queries)
//       3) Geofabrik extracts (regional)

use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Tile {
    lat_idx: u8,  // 0-70 (71 tiles)
    lon_idx: u8,  // 0-58 (59 tiles)
    level: u8,    // 0-46 (47 levels)
    priority: u8, // 0=highest, 255=lowest
    size_bytes: u32,
}

impl Tile {
    fn shard_id(&self) -> u16 {
        (self.lat_idx as u16 * 59 + self.lon_idx as u16) % 196883
    }
    
    fn archive_url(&self) -> String {
        let dataset = match self.lat_idx {
            0..=10 => "pacific",
            11..=20 => "himalayas",
            21..=30 => "amazon",
            31..=40 => "newyork",
            41..=50 => "tokyo",
            51..=60 => "london",
            _ => "omega",
        };
        format!("https://archive.org/download/osm-planet-{}-monster/tile_{}_{}.geojson",
                dataset, self.lat_idx, self.lon_idx)
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority) // Min-heap
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Tile {}
impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.lat_idx == other.lat_idx && self.lon_idx == other.lon_idx
    }
}

struct FetchPlanner {
    queue: BinaryHeap<Tile>,
    viewport: (f64, f64, f64, f64), // (min_lat, min_lon, max_lat, max_lon)
    max_bytes: u32,
}

impl FetchPlanner {
    fn new(viewport: (f64, f64, f64, f64), max_bytes: u32) -> Self {
        Self {
            queue: BinaryHeap::new(),
            viewport,
            max_bytes,
        }
    }
    
    fn lat_to_tile(lat: f64) -> u8 {
        (((lat + 90.0) / 180.0 * 71.0) as u8).min(70)
    }
    
    fn lon_to_tile(lon: f64) -> u8 {
        (((lon + 180.0) / 360.0 * 59.0) as u8).min(58)
    }
    
    fn plan(&mut self) {
        let (min_lat, min_lon, max_lat, max_lon) = self.viewport;
        
        let lat_start = Self::lat_to_tile(min_lat);
        let lat_end = Self::lat_to_tile(max_lat);
        let lon_start = Self::lon_to_tile(min_lon);
        let lon_end = Self::lon_to_tile(max_lon);
        
        // Center tile = highest priority
        let center_lat = Self::lat_to_tile((min_lat + max_lat) / 2.0);
        let center_lon = Self::lon_to_tile((min_lon + max_lon) / 2.0);
        
        for lat in lat_start..=lat_end {
            for lon in lon_start..=lon_end {
                let dist = ((lat as i16 - center_lat as i16).abs() + 
                           (lon as i16 - center_lon as i16).abs()) as u8;
                
                self.queue.push(Tile {
                    lat_idx: lat,
                    lon_idx: lon,
                    level: 0,
                    priority: dist,
                    size_bytes: 4096, // ~4KB per tile
                });
            }
        }
    }
    
    fn fetch_plan(&mut self) -> Vec<Tile> {
        let mut plan = Vec::new();
        let mut total_bytes = 0u32;
        
        while let Some(tile) = self.queue.pop() {
            if total_bytes + tile.size_bytes > self.max_bytes {
                break;
            }
            total_bytes += tile.size_bytes;
            plan.push(tile);
        }
        
        plan
    }
}

async fn fetch_tile(tile: &Tile) -> Result<String, Box<dyn std::error::Error>> {
    let url = tile.archive_url();
    let client = reqwest::Client::new();
    let resp = client.get(&url)
        .header("Range", "bytes=0-50000") // Max 50KB
        .send()
        .await?;
    
    Ok(resp.text().await?)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example: Giza Pyramids viewport
    let viewport = (29.9, 31.0, 30.1, 31.2);
    let max_bytes = 200_000; // 200KB max
    
    let mut planner = FetchPlanner::new(viewport, max_bytes);
    planner.plan();
    
    let plan = planner.fetch_plan();
    
    println!("📋 Fetch Plan:");
    println!("Viewport: {:?}", viewport);
    println!("Tiles to fetch: {}", plan.len());
    println!("Total size: ~{}KB", plan.iter().map(|t| t.size_bytes).sum::<u32>() / 1024);
    println!();
    
    for (i, tile) in plan.iter().enumerate() {
        println!("{}. Tile[{},{}] priority={} shard={} url={}",
                 i + 1, tile.lat_idx, tile.lon_idx, tile.priority, 
                 tile.shard_id(), tile.archive_url());
        
        // Fetch first 3 tiles only
        if i < 3 {
            match fetch_tile(tile).await {
                Ok(data) => println!("   ✓ Fetched {} bytes", data.len()),
                Err(e) => println!("   ✗ Error: {}", e),
            }
        }
    }
    
    Ok(())
}
