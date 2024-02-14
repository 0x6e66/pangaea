use elasticsearch::{http::transport::Transport, Elasticsearch};
use pangaea::dataset::datasettype::Dataset;

fn get_client() -> Elasticsearch {
    let transport = Transport::single_node("https://ws.pangaea.de/es/pangaea").unwrap();
    Elasticsearch::new(transport)
}

#[tokio::test]
async fn dataset_new() {
    let dataset_id = 82037;

    let _ = Dataset::new(dataset_id, &get_client()).await.unwrap();
}

#[tokio::test]
async fn dataset_search() {
    let _ = Dataset::search(0, 10, None, &["sp-lastModified:desc"], &get_client())
        .await
        .unwrap();
}
