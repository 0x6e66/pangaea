//! Interoperability with the elasticsearch index from PANGAEA

use crate::{prelude::*, types::MetaDataType};
use elasticsearch::{http::transport::Transport, Elasticsearch, GetParts, SearchParts};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ElasticSearchResponse {
    took: u32,
    timed_out: bool,
    _shards: Shards,
    hits: Hits,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Shards {
    total: u32,
    successful: u32,
    skipped: u32,
    failed: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Hits {
    total: u32,
    max_score: Option<f32>,
    hits: Vec<SearchHit>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct SearchHit {
    _index: String,
    _type: String,
    _id: String,
    _score: Option<f32>,
    _source: Value,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct IndexHit {
    _index: String,
    _type: String,
    _id: String,
    _version: i32,
    found: bool,
    _source: Value,
}

/// Search for datasets in the elasticsearch index.
///
/// Note: `from` + `size` must be less than or equal to 10000
pub async fn elastic_search(
    from: i64,
    size: i64,
    body: Option<Value>,
    sort: &[&str],
) -> Result<Vec<Result<MetaDataType>>> {
    let transport = Transport::single_node("https://ws.pangaea.de/es/pangaea")?;
    let client = Elasticsearch::new(transport);

    let response = client
        .search(SearchParts::Index(&["panmd"]))
        .from(from)
        .size(size)
        .body(body.unwrap_or(json!({})))
        .sort(sort)
        .send()
        .await?;

    let text = response.text().await?;
    let response: ElasticSearchResponse = serde_json::from_str(&text)?;

    Ok(response
        .hits
        .hits
        .into_iter()
        .map(|hit| match hit._source["xml"].as_str() {
            None => Err(Error::Generic(
                "XML-Element of response is no string".to_owned(),
            )),
            Some(xml) => yaserde::de::from_str::<MetaDataType>(xml).map_err(Error::Generic),
        })
        .collect())
}

/// Get metadata over a single dataset with the id `dataset_id`
pub async fn elastic_doc(dataset_id: u32) -> Result<MetaDataType> {
    let transport = Transport::single_node("https://ws.pangaea.de/es/pangaea")?;
    let client = Elasticsearch::new(transport);

    let response = client
        .get(GetParts::IndexId("panmd", format!("{dataset_id}").as_str()))
        .send()
        .await?;

    let text = response.text().await?;
    let hit = serde_json::from_str::<IndexHit>(&text)?;

    match hit._source["xml"].as_str() {
        None => Err(Error::Generic(
            "XML-Element of response is no String".to_owned(),
        )),
        Some(xml) => yaserde::de::from_str::<MetaDataType>(xml).map_err(Error::Generic),
    }
}
