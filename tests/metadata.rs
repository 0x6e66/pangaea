use elasticsearch::{http::transport::Transport, Elasticsearch};
use pangaea::metadata::metadatatype::MetaDataType;

fn get_client() -> Elasticsearch {
    let transport = Transport::single_node("https://ws.pangaea.de/es/pangaea").unwrap();
    Elasticsearch::new(transport)
}

#[tokio::test]
async fn metadata_new() {
    let dataset_id = 82037;

    let _ = MetaDataType::new(dataset_id, &get_client()).await.unwrap();
}

#[tokio::test]
async fn metadata_search() {
    let _ = MetaDataType::search(0, 10, None, &["sp-lastModified:desc"], &get_client())
        .await
        .unwrap();
}
