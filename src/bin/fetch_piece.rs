// Phase 2: Fetch specific piece and print data
use osm_planet_torrent::piece_download::download_specific_piece;
use osm_planet_torrent::print_storage::PrintStorageFactory;
use librqbit::storage::StorageFactoryExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: fetch-piece <piece_id>");
        eprintln!("Example: fetch-piece 13668");
        std::process::exit(1);
    }
    
    let piece_id: u32 = args[1].parse()?;
    
    println!("🔍 Phase 2: Fetching piece {} with PrintStorage...", piece_id);
    
    // Use custom storage that prints instead of writing 80GB file
    let storage = PrintStorageFactory { target_piece: piece_id }.boxed();
    
    download_specific_piece(
        "osm-planet.torrent",
        "./osm-data",
        piece_id,
        Some(storage),
    ).await?;
    
    println!("✓ Piece {} data captured in ./chunks/", piece_id);
    
    Ok(())
}
