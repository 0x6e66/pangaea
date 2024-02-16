//! Metadata type, that contains every information available to a pangaea-dataset

use elasticsearch::{Elasticsearch, GetParts, SearchParts};
use serde_json::{json, Value};

use crate::prelude::*;

pub(crate) mod metadatatype;

use self::metadatatype::{ElasticSearchResponse, IndexHit};
pub use metadatatype::MetaDataType;

impl MetaDataType {
    /// Get metadata over a single dataset with the id `dataset_id`
    pub async fn new(dataset_id: u32, client: &Elasticsearch) -> Result<MetaDataType> {
        let response = client
            .get(GetParts::IndexId("panmd", format!("{dataset_id}").as_str()))
            .send()
            .await?;

        if response.status_code() != 200 {
            return Err(Error::Generic(format!(
                "HTTP Response code was '{}'",
                response.status_code()
            )));
        }

        let text = response.text().await?;
        let hit = serde_json::from_str::<IndexHit>(&text)?;

        match hit._source["xml"].as_str() {
            None => Err(Error::Generic(
                "XML-Element of response is no String".to_owned(),
            )),
            Some(xml) => {
                yaserde::de::from_str::<metadatatype::MetaDataType>(xml).map_err(Error::Generic)
            }
        }
    }

    /// Search for datasets in the elasticsearch index.
    ///
    /// Note: `from` + `size` must be less than or equal to 10000
    pub async fn search(
        from: i64,
        size: i64,
        body: Option<Value>,
        sort: &[&str],
        client: &Elasticsearch,
    ) -> Result<Vec<Result<MetaDataType>>> {
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
                Some(xml) => {
                    yaserde::de::from_str::<metadatatype::MetaDataType>(xml).map_err(Error::Generic)
                }
            })
            .collect())
    }
}
