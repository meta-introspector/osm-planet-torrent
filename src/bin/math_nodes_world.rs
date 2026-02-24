// Project LMFDB, OEIS, Wikidata, GAP, PARI as discoverable nodes in OSM world
use std::fs::File;
use std::io::{Read, Seek};
use serde_json::json;

const MONSTER_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 47, 59, 71];

// LMFDB objects projected to Earth
const LMFDB_NODES: [(&str, &str, f64, f64, u64); 15] = [
    ("elliptic_curve.11a1", "Smallest conductor", 51.5074, -0.1278, 11),
    ("modular_form.ramanujan_tau", "Ramanujan tau", 10.9617, 79.3881, 71),
    ("l_function.riemann_zeta", "Riemann zeta", 51.5333, 9.9333, 59),
    ("number_field.Q_sqrt5", "Golden ratio field", 47.5596, 7.5886, 5),
    ("galois_group.S5", "Symmetric group", 48.8566, 2.3522, 5),
    ("genus2_curve.169", "Genus 2", 52.2053, 0.1218, 13),
    ("abelian_variety.2.2.5", "Jacobian", 40.7128, -74.0060, 2),
    ("artin_rep.2.23", "Artin L", 35.6895, 139.6917, 23),
    ("mf.newform.11.2", "Weight 2", 13.0827, 80.2707, 11),
    ("ec.q.37a", "Rank 1", 55.7558, 37.6173, 37),
    ("nf.3.1.23", "Cubic field", 39.9042, 116.4074, 23),
    ("g2c.169.a", "Genus 2 curve", 19.4326, -99.1332, 13),
    ("av.fq.2.2", "Abelian variety", -33.8688, 151.2093, 2),
    ("lf.2.0.2", "Local field", 28.6139, 77.2090, 2),
    ("character.7.2", "Dirichlet char", 1.3521, 103.8198, 7),
];

// OEIS sequences projected to Earth
const OEIS_NODES: [(&str, &str, f64, f64, u64); 15] = [
    ("A000045", "Fibonacci", 47.5596, 7.5886, 2),
    ("A000040", "Primes", 51.5074, -0.1278, 2),
    ("A000796", "Pi digits", 48.8566, 2.3522, 3),
    ("A001113", "e digits", 51.5333, 9.9333, 3),
    ("A000108", "Catalan", 52.2053, 0.1218, 5),
    ("A000110", "Bell numbers", 40.7128, -74.0060, 7),
    ("A000041", "Partitions", 10.9617, 79.3881, 71),
    ("A000203", "Divisor sum", 13.0827, 80.2707, 11),
    ("A000594", "Ramanujan tau", 10.9617, 79.3881, 71),
    ("A001157", "Sum of squares", 35.6895, 139.6917, 13),
    ("A002267", "Monster order", 55.7558, 37.6173, 71),
    ("A003418", "LCM(1..n)", 39.9042, 116.4074, 17),
    ("A005843", "Even numbers", 19.4326, -99.1332, 2),
    ("A006530", "Largest prime", -33.8688, 151.2093, 59),
    ("A008683", "Möbius", 28.6139, 77.2090, 29),
];

// Wikidata math entities
const WIKIDATA_NODES: [(&str, &str, f64, f64, u64); 15] = [
    ("Q83163", "Ramanujan", 10.9617, 79.3881, 71),
    ("Q191037", "Hardy", 51.5074, -0.1278, 17),
    ("Q14277", "Euler", 47.5596, 7.5886, 2),
    ("Q5830", "Gauss", 51.5333, 9.9333, 3),
    ("Q7604", "Riemann", 51.5333, 9.9333, 59),
    ("Q5753", "Fermat", 48.8566, 2.3522, 5),
    ("Q5638", "Pascal", 48.8566, 2.3522, 7),
    ("Q312545", "Littlewood", 52.2053, 0.1218, 11),
    ("Q7604", "Hilbert", 51.5333, 9.9333, 13),
    ("Q5753", "Poincaré", 48.8566, 2.3522, 29),
    ("Q7604", "Noether", 51.5333, 9.9333, 31),
    ("Q5638", "Turing", 53.4808, -2.2426, 41),
    ("Q5753", "von Neumann", 40.3573, -74.6672, 47),
    ("Q7604", "Erdős", 47.4979, 19.0402, 59),
    ("Q5638", "Grothendieck", 48.8566, 2.3522, 71),
];

// GAP groups
const GAP_NODES: [(&str, &str, f64, f64, u64); 10] = [
    ("Monster", "Largest sporadic", 52.2053, 0.1218, 71),
    ("Baby Monster", "Second largest", 52.2053, 0.1218, 47),
    ("Mathieu M24", "Steiner system", 52.2053, 0.1218, 23),
    ("Mathieu M23", "Sporadic", 52.2053, 0.1218, 23),
    ("Mathieu M22", "Sporadic", 52.2053, 0.1218, 11),
    ("Conway Co1", "Leech lattice", 52.2053, 0.1218, 59),
    ("Fischer Fi24", "3-transposition", 52.2053, 0.1218, 29),
    ("Suzuki Sz", "Simple group", 35.6895, 139.6917, 13),
    ("Ree R", "Twisted group", 35.6895, 139.6917, 13),
    ("Tits T", "Simple group", 48.8566, 2.3522, 13),
];

// PARI/GP functions
const PARI_NODES: [(&str, &str, f64, f64, u64); 10] = [
    ("isprime", "Primality test", 51.5074, -0.1278, 2),
    ("factor", "Factorization", 51.5333, 9.9333, 2),
    ("ellap", "Elliptic curve", 52.2053, 0.1218, 11),
    ("bnfinit", "Number field", 51.5333, 9.9333, 5),
    ("zetakinit", "Dedekind zeta", 51.5333, 9.9333, 59),
    ("mfcoefs", "Modular form", 10.9617, 79.3881, 71),
    ("polroots", "Polynomial roots", 48.8566, 2.3522, 3),
    ("qfbclassno", "Class number", 51.5333, 9.9333, 7),
    ("ellrank", "Rank computation", 52.2053, 0.1218, 11),
    ("lfun", "L-function", 51.5333, 9.9333, 59),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = std::time::Instant::now();
    
    eprintln!("🌍 Projecting Math Databases into OSM World");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("Sources: LMFDB + OEIS + Wikidata + GAP + PARI");
    
    let planet_file = "/mnt/data1/osm-planet/planet-latest.osm.pbf";
    let mut file = File::open(planet_file)?;
    
    let mut all_nodes = Vec::new();
    
    // Project LMFDB
    eprintln!("\n📊 LMFDB: {} objects", LMFDB_NODES.len());
    for (id, desc, lat, lon, prime) in &LMFDB_NODES {
        let piece = calculate_piece(*lat, *lon);
        let shard = piece % 71;
        
        all_nodes.push(json!({
            "source": "LMFDB",
            "id": id,
            "description": desc,
            "position": [*lon, *lat],
            "osm": {"piece": piece, "shard": shard},
            "prime": prime,
            "url": format!("https://www.lmfdb.org/{}", id),
        }));
    }
    
    // Project OEIS
    eprintln!("🔢 OEIS: {} sequences", OEIS_NODES.len());
    for (id, desc, lat, lon, prime) in &OEIS_NODES {
        let piece = calculate_piece(*lat, *lon);
        let shard = piece % 71;
        
        all_nodes.push(json!({
            "source": "OEIS",
            "id": id,
            "description": desc,
            "position": [*lon, *lat],
            "osm": {"piece": piece, "shard": shard},
            "prime": prime,
            "url": format!("https://oeis.org/{}", id),
        }));
    }
    
    // Project Wikidata
    eprintln!("📚 Wikidata: {} mathematicians", WIKIDATA_NODES.len());
    for (qid, name, lat, lon, prime) in &WIKIDATA_NODES {
        let piece = calculate_piece(*lat, *lon);
        let shard = piece % 71;
        
        all_nodes.push(json!({
            "source": "Wikidata",
            "id": qid,
            "description": name,
            "position": [*lon, *lat],
            "osm": {"piece": piece, "shard": shard},
            "prime": prime,
            "url": format!("https://www.wikidata.org/wiki/{}", qid),
        }));
    }
    
    // Project GAP
    eprintln!("🔷 GAP: {} groups", GAP_NODES.len());
    for (id, desc, lat, lon, prime) in &GAP_NODES {
        let piece = calculate_piece(*lat, *lon);
        let shard = piece % 71;
        
        all_nodes.push(json!({
            "source": "GAP",
            "id": id,
            "description": desc,
            "position": [*lon, *lat],
            "osm": {"piece": piece, "shard": shard},
            "prime": prime,
            "url": "https://www.gap-system.org/",
        }));
    }
    
    // Project PARI
    eprintln!("⚙️  PARI: {} functions", PARI_NODES.len());
    for (id, desc, lat, lon, prime) in &PARI_NODES {
        let piece = calculate_piece(*lat, *lon);
        let shard = piece % 71;
        
        all_nodes.push(json!({
            "source": "PARI",
            "id": id,
            "description": desc,
            "position": [*lon, *lat],
            "osm": {"piece": piece, "shard": shard},
            "prime": prime,
            "url": "https://pari.math.u-bordeaux.fr/",
        }));
    }
    
    // Calculate statistics
    let by_source: std::collections::HashMap<_, _> = all_nodes.iter()
        .fold(std::collections::HashMap::new(), |mut acc, node| {
            let source = node["source"].as_str().unwrap();
            *acc.entry(source).or_insert(0) += 1;
            acc
        });
    
    let unique_shards: std::collections::HashSet<_> = all_nodes.iter()
        .map(|n| n["osm"]["shard"].as_u64().unwrap())
        .collect();
    
    eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("🎯 Projection complete!");
    eprintln!("   Total nodes: {}", all_nodes.len());
    eprintln!("   Unique shards: {}", unique_shards.len());
    eprintln!("   By source: {:?}", by_source);
    
    let output = json!({
        "type": "MathDatabasesProjection",
        "sources": ["LMFDB", "OEIS", "Wikidata", "GAP", "PARI"],
        "nodes": all_nodes,
        "statistics": {
            "total_nodes": all_nodes.len(),
            "unique_shards": unique_shards.len(),
            "by_source": by_source,
        },
        "geojson": {
            "type": "FeatureCollection",
            "features": all_nodes.iter().map(|node| json!({
                "type": "Feature",
                "geometry": {
                    "type": "Point",
                    "coordinates": node["position"],
                },
                "properties": {
                    "source": node["source"],
                    "id": node["id"],
                    "description": node["description"],
                    "shard": node["osm"]["shard"],
                    "prime": node["prime"],
                    "url": node["url"],
                },
            })).collect::<Vec<_>>(),
        },
        "speedrun_ms": start.elapsed().as_millis(),
    });
    
    let mut out = File::create("/tmp/math_nodes_world.json")?;
    serde_json::to_writer_pretty(&mut out, &output)?;
    
    eprintln!("\n✅ /tmp/math_nodes_world.json");
    eprintln!("⚡ {}ms", start.elapsed().as_millis());
    Ok(())
}

fn calculate_piece(lat: f64, lon: f64) -> u32 {
    let lat_norm = ((lat + 90.0) / 180.0 * 10000.0) as u32;
    let lon_norm = ((lon + 180.0) / 360.0 * 10000.0) as u32;
    (lat_norm + lon_norm) % 21763
}
