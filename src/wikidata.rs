use reqwest;
use serde_json::Value;

pub async fn query_wikidata(qid: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let sparql = format!(
        r#"
        SELECT ?item ?itemLabel ?coord ?article WHERE {{
          VALUES ?item {{ wd:{} }}
          OPTIONAL {{ ?item wdt:P625 ?coord }}
          OPTIONAL {{ ?article schema:about ?item ; schema:isPartOf <https://en.wikipedia.org/> }}
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
    Ok(data)
}

pub async fn get_linked_entities(qid: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let sparql = format!(
        r#"
        SELECT DISTINCT ?linked WHERE {{
          wd:{} ?p ?linked .
          FILTER(STRSTARTS(STR(?linked), "http://www.wikidata.org/entity/Q"))
        }}
        LIMIT 17
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
    let mut entities = Vec::new();
    
    if let Some(bindings) = data["results"]["bindings"].as_array() {
        for binding in bindings {
            if let Some(uri) = binding["linked"]["value"].as_str() {
                if let Some(qid) = uri.strip_prefix("http://www.wikidata.org/entity/") {
                    entities.push(qid.to_string());
                }
            }
        }
    }
    
    Ok(entities)
}
