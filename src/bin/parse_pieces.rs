// Parse reconstructed PBF pieces and extract OSM data
use osmpbf::{Element, ElementReader};
use std::fs;

fn main() -> anyhow::Result<()> {
    let pieces = vec![
        "piece_0000001_reconstructed.pbf",
        "piece_0000002_reconstructed.pbf", 
        "piece_0000003_reconstructed.pbf",
        "piece_0000004_reconstructed.pbf",
    ];
    
    for piece_file in pieces {
        if !std::path::Path::new(piece_file).exists() {
            continue;
        }
        
        println!("\n📦 Parsing {}", piece_file);
        let data = fs::read(piece_file)?;
        
        let reader = ElementReader::new(std::io::Cursor::new(&data));
        
        let mut nodes = 0;
        let mut ways = 0;
        let mut relations = 0;
        let mut wikidata_entities = Vec::new();
        
        for element in reader.par_map_reduce(
            |element| {
                let mut n = 0;
                let mut w = 0;
                let mut r = 0;
                let mut wd = Vec::new();
                
                match element {
                    Element::Node(node) => {
                        n = 1;
                        for (key, val) in node.tags() {
                            if key == "wikidata" {
                                let name = node.tags()
                                    .find(|(k, _)| *k == "name")
                                    .map(|(_, v)| v.to_string())
                                    .unwrap_or_default();
                                wd.push((name, val.to_string(), node.lat(), node.lon()));
                            }
                        }
                    }
                    Element::Way(_) => w = 1,
                    Element::Relation(_) => r = 1,
                    _ => {}
                }
                (n, w, r, wd)
            },
            || (0, 0, 0, Vec::new()),
            |mut a, b| {
                a.0 += b.0;
                a.1 += b.1;
                a.2 += b.2;
                a.3.extend(b.3);
                a
            }
        ) {
            nodes += element.0;
            ways += element.1;
            relations += element.2;
            wikidata_entities.extend(element.3);
        }
        
        println!("   ✓ Nodes: {}", nodes);
        println!("   ✓ Ways: {}", ways);
        println!("   ✓ Relations: {}", relations);
        println!("   ✓ Wikidata entities: {}", wikidata_entities.len());
        
        // Show first 10 wikidata entities
        for (name, qid, lat, lon) in wikidata_entities.iter().take(10) {
            println!("     {} ({}) at {:.4}, {:.4}", name, qid, lat, lon);
        }
    }
    
    Ok(())
}
