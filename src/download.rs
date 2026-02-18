use std::process::Command;
use std::fs;

pub fn download_pieces(torrent_file: &str, pieces: &[usize], output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(output_dir)?;
    
    // Generate aria2c command for selective download
    let piece_list: Vec<String> = pieces.iter().map(|p| (p + 1).to_string()).collect();
    let select_file = piece_list.join(",");
    
    println!("📥 Downloading {} pieces...", pieces.len());
    println!("  Pieces: {}", if pieces.len() > 10 {
        format!("{}...{}", &piece_list[..5].join(","), &piece_list[pieces.len()-5..].join(","))
    } else {
        select_file.clone()
    });
    
    let output = Command::new("aria2c")
        .arg("--select-file").arg(&select_file)
        .arg("--dir").arg(output_dir)
        .arg("--seed-time=0")
        .arg("--max-connection-per-server=16")
        .arg("--split=16")
        .arg(torrent_file)
        .output()?;
    
    if !output.status.success() {
        return Err(format!("aria2c failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    
    println!("✓ Downloaded {} pieces to {}", pieces.len(), output_dir);
    Ok(())
}

pub fn extract_wikidata_from_osm(pbf_file: &str) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    // Use osmium to extract wikidata tags
    let output = Command::new("osmium")
        .arg("tags-filter")
        .arg(pbf_file)
        .arg("nwr/wikidata")
        .arg("-o")
        .arg("-")
        .arg("-f")
        .arg("json")
        .output()?;
    
    if !output.status.success() {
        return Err("osmium failed".into());
    }
    
    // Parse JSON output for wikidata tags
    let json_str = String::from_utf8(output.stdout)?;
    let mut wikidata_tags = Vec::new();
    
    for line in json_str.lines() {
        if let Ok(obj) = serde_json::from_str::<serde_json::Value>(line) {
            if let Some(tags) = obj["properties"]["tags"].as_object() {
                if let Some(qid) = tags.get("wikidata").and_then(|v| v.as_str()) {
                    if let Some(name) = tags.get("name").and_then(|v| v.as_str()) {
                        wikidata_tags.push((name.to_string(), qid.to_string()));
                    }
                }
            }
        }
    }
    
    Ok(wikidata_tags)
}
