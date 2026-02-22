// Parse one PBF block and show nodes
use anyhow::Result;
use osmpbf::{Element, ElementReader};

fn main() -> Result<()> {
    let piece_file = "piece_9055.pbf";
    
    println!("🔍 Parsing {}", piece_file);
    println!();
    
    let reader = ElementReader::from_path(piece_file)?;
    
    let mut count = 0;
    let mut shown = 0;
    
    for element in reader.par_map_reduce(
        |element| {
            let mut nodes = Vec::new();
            match element {
                Element::Node(node) => {
                    nodes.push((node.id(), node.lat(), node.lon(), 
                        node.tags().map(|(k,v)| (k.to_string(), v.to_string())).collect::<Vec<_>>()));
                }
                _ => {}
            }
            nodes
        },
        Vec::new,
        |mut a, b| { a.extend(b); a }
    ) {
        for (id, lat, lon, tags) in element {
            count += 1;
            
            if shown < 20 {
                println!("Node {}: ({:.4}, {:.4})", id, lat, lon);
                if !tags.is_empty() {
                    for (k, v) in tags.iter().take(3) {
                        println!("  {}: {}", k, v);
                    }
                }
                shown += 1;
            }
        }
    }
    
    println!();
    println!("✅ Total nodes: {}", count);
    
    Ok(())
}
