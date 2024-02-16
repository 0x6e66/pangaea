//! Striped down version of Metadata

use elasticsearch::Elasticsearch;
use serde_json::Value;

pub(crate) mod datasettype;

pub use self::datasettype::Dataset;
use crate::{metadata::MetaDataType, prelude::*};

impl Dataset {
    /// Get metadata over a single dataset with the id `dataset_id`
    pub async fn new(dataset_id: u32, client: &Elasticsearch) -> Result<Dataset> {
        MetaDataType::new(dataset_id, client)
            .await
            .map(|md| md.into())
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
    ) -> Result<Vec<Result<Dataset>>> {
        MetaDataType::search(from, size, body, sort, client)
            .await
            .map(|vec| vec.into_iter().map(|res| res.map(|md| md.into())).collect())
    }
}
