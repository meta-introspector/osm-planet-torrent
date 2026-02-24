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
        println!("\n📦 Parsing {}", piece_file);
        let data = fs::read(piece_file)?;
        
        let reader = ElementReader::new(std::io::Cursor::new(&data));
        
        let mut nodes = 0;
        let mut ways = 0;
        let mut relations = 0;
        let mut wikidata_count = 0;
        
        for element in reader.par_map_reduce(
            |element| {
                let mut n = 0;
                let mut w = 0;
                let mut r = 0;
                let mut wd = 0;
                
                match element {
                    Element::Node(node) => {
                        n = 1;
                        for (key, val) in node.tags() {
                            if key == "wikidata" {
                                wd = 1;
                                println!("   🎯 Node {} has wikidata: {}", node.id(), val);
                            }
                        }
                    }
                    Element::Way(_) => w = 1,
                    Element::Relation(_) => r = 1,
                    _ => {}
                }
                (n, w, r, wd)
            },
            || (0, 0, 0, 0),
            |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3)
        ) {
            nodes += element.0;
            ways += element.1;
            relations += element.2;
            wikidata_count += element.3;
        }
        
        println!("   Nodes: {}", nodes);
        println!("   Ways: {}", ways);
        println!("   Relations: {}", relations);
        println!("   With Wikidata: {}", wikidata_count);
    }
    
    Ok(())
}
