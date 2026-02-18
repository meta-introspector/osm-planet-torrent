use reqwest;
use serde_json::Value;
use std::collections::HashSet;

pub async fn extract_wikidata_from_results(wikidata_file: &str) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(wikidata_file)?;
    let json: Value = serde_json::from_str(&data)?;
    
    let mut all_qids = HashSet::new();
    
    // Extract from queries
    if let Some(queries) = json["queries"].as_array() {
        for query in queries {
            if let Some(qid) = query["qid"].as_str() {
                all_qids.insert(qid.to_string());
            }
            
            // Extract linked entities
            if let Some(linked) = query["linked"].as_array() {
                for link in linked {
                    if let Some(qid) = link.as_str() {
                        all_qids.insert(qid.to_string());
                    }
                }
            }
        }
    }
    
    Ok(all_qids)
}

pub async fn get_wikipedia_articles(qid: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let sparql = format!(
        r#"
        SELECT ?article ?articleLabel WHERE {{
          ?article schema:about wd:{} .
          ?article schema:isPartOf ?site .
          FILTER(CONTAINS(STR(?site), "wikipedia"))
          SERVICE wikibase:label {{ bd:serviceParam wikibase:language "en" }}
        }}
        "#,
        qid
    );
    
    let client = reqwest::Client::new();
    let response = client
        .get("https://query.wikidata.org/sparql")
        .query(&[("query", sparql), ("format", "json".to_string())])
        .header("User-Agent", "osm-planet-torrent/1.0")
        .send()
        .await?;
    
    let data: Value = response.json().await?;
    let mut articles = Vec::new();
    
    if let Some(bindings) = data["results"]["bindings"].as_array() {
        for binding in bindings {
            if let Some(url) = binding["article"]["value"].as_str() {
                articles.push(url.to_string());
            }
        }
    }
    
    Ok(articles)
}

pub async fn recursive_wikidata_crawl(
    initial_qids: HashSet<String>,
    max_depth: usize
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut all_data = serde_json::json!({
        "initial_qids": initial_qids.len(),
        "max_depth": max_depth,
        "entities": {}
    });
    
    let mut visited = HashSet::new();
    let mut current_level = initial_qids;
    
    for depth in 0..max_depth {
        println!("  Depth {}: {} entities to process", depth, current_level.len());
        
        let mut next_level = HashSet::new();
        
        for qid in &current_level {
            if visited.contains(qid) {
                continue;
            }
            
            visited.insert(qid.clone());
            
            // Query entity
            match super::wikidata::query_wikidata(qid).await {
                Ok(data) => {
                    all_data["entities"][qid] = data;
                    
                    // Get Wikipedia articles
                    if let Ok(articles) = get_wikipedia_articles(qid).await {
                        all_data["entities"][qid]["wikipedia_articles"] = serde_json::json!(articles);
                    }
                    
                    // Get linked entities for next level
                    if depth < max_depth - 1 {
                        if let Ok(linked) = super::wikidata::get_linked_entities(qid).await {
                            for link in linked {
                                if !visited.contains(&link) {
                                    next_level.insert(link);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("    ✗ Error querying {}: {}", qid, e);
                }
            }
            
            // Rate limit
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        current_level = next_level;
        
        if current_level.is_empty() {
            break;
        }
    }
    
    all_data["total_entities"] = serde_json::json!(visited.len());
    
    Ok(all_data)
}
