// Custom storage that builds spatial index as chunks arrive
use librqbit::storage::{StorageFactory, TorrentStorage};
use librqbit::torrent_state::{ManagedTorrentShared, TorrentMetadata};
use anyhow::Result;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;
use osmpbf::{Element, ElementReader};

pub struct PrintStorage {
    target_piece: u32,
}

impl PrintStorage {
    fn extract_bbox(&self, data: &[u8]) -> Option<(f64, f64, f64, f64)> {
        let reader = ElementReader::new(std::io::Cursor::new(data));
        
        let mut min_lat = f64::MAX;
        let mut max_lat = f64::MIN;
        let mut min_lon = f64::MAX;
        let mut max_lon = f64::MIN;
        let mut found = false;
        
        for element in reader.par_map_reduce(
            |element| {
                let mut coords = Vec::new();
                match element {
                    Element::Node(node) => {
                        coords.push((node.lat(), node.lon()));
                    }
                    _ => {}
                }
                coords
            },
            || Vec::new(),
            |mut a, b| { a.extend(b); a }
        ) {
            for (lat, lon) in element {
                min_lat = min_lat.min(lat);
                max_lat = max_lat.max(lat);
                min_lon = min_lon.min(lon);
                max_lon = max_lon.max(lon);
                found = true;
            }
        }
        
        if found {
            Some((min_lat, max_lat, min_lon, max_lon))
        } else {
            None
        }
    }
}

impl TorrentStorage for PrintStorage {
    fn pwrite_all(&self, file_id: usize, offset: u64, buf: &[u8]) -> Result<()> {
        let piece_size = 4194304u64;
        let piece_id = offset / piece_size;
        
        println!("📝 Piece {}: offset={}, size={} bytes", piece_id, offset, buf.len());
        
        // Save ALL chunks to disk
        let chunk_file = format!("./chunks/piece_{:07}_offset_{:010}.bin", piece_id, offset);
        std::fs::create_dir_all("./chunks").ok();
        std::fs::write(&chunk_file, buf).ok();
        
        // Try to parse OSM data and extract bounding box
        if buf.len() > 100 {
            if let Some((min_lat, max_lat, min_lon, max_lon)) = self.extract_bbox(buf) {
                println!("   📍 BBox: lat[{:.4}, {:.4}], lon[{:.4}, {:.4}]", 
                    min_lat, max_lat, min_lon, max_lon);
                
                // Append to spatial index
                let mut index = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("spatial_index.jsonl")?;
                
                writeln!(index, "{{\"piece\":{},\"offset\":{},\"size\":{},\"lat_min\":{},\"lat_max\":{},\"lon_min\":{},\"lon_max\":{}}}",
                    piece_id, offset, buf.len(), min_lat, max_lat, min_lon, max_lon)?;
            }
        }
        
        // Highlight target piece
        if piece_id == self.target_piece as u64 {
            println!("   🎯 TARGET PIECE!");
        }
        
        Ok(())
    }

    fn pread_exact(&self, _file_id: usize, _offset: u64, _buf: &mut [u8]) -> Result<()> {
        Ok(())
    }

    fn remove_file(&self, _file_id: usize, _filename: &Path) -> Result<()> {
        Ok(())
    }

    fn remove_directory_if_empty(&self, _path: &Path) -> Result<()> {
        Ok(())
    }

    fn ensure_file_length(&self, _file_id: usize, _length: u64) -> Result<()> {
        Ok(())
    }

    fn take(&self) -> Result<Box<dyn TorrentStorage>> {
        Ok(Box::new(PrintStorage { target_piece: self.target_piece }))
    }

    fn init(&mut self, _shared: &ManagedTorrentShared, _meta: &TorrentMetadata) -> Result<()> {
        println!("🔧 Building spatial index as chunks arrive...");
        println!("   Target piece: {} (will be highlighted)", self.target_piece);
        println!("   Saving all chunks to ./chunks/");
        println!("   Writing spatial index to spatial_index.jsonl");
        Ok(())
    }
}

#[derive(Clone)]
pub struct PrintStorageFactory {
    pub target_piece: u32,
}

impl StorageFactory for PrintStorageFactory {
    type Storage = Box<dyn TorrentStorage>;

    fn create(&self, _shared: &ManagedTorrentShared, _metadata: &TorrentMetadata) -> Result<Self::Storage> {
        Ok(Box::new(PrintStorage {
            target_piece: self.target_piece,
        }))
    }

    fn clone_box(&self) -> librqbit::storage::BoxStorageFactory {
        Box::new(self.clone())
    }
}
